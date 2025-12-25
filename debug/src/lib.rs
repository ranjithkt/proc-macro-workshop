use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parse_macro_input, parse_quote, Attribute, DeriveInput, GenericArgument, Ident, Lit, Meta,
    PathArguments, Type, TypePath, WherePredicate,
};

/// Field information parsed by darling
#[derive(FromField)]
#[darling(forward_attrs(debug))]
struct DebugField {
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<Attribute>,
}

impl DebugField {
    /// Get the custom format from #[debug = "..."] attribute
    fn get_format(&self) -> Option<String> {
        for attr in &self.attrs {
            if !attr.path().is_ident("debug") {
                continue;
            }
            // Handle #[debug = "..."] format (MetaNameValue)
            if let Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    return Some(lit_str.value());
                }
            }
        }
        None
    }
}

/// Derive input parsed by darling (no attributes parsing at struct level)
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), forward_attrs(debug))]
struct DebugInput {
    ident: Ident,
    generics: syn::Generics,
    data: Data<(), DebugField>,
    attrs: Vec<Attribute>,
}

impl DebugInput {
    /// Get the struct-level bound from #[debug(bound = "...")] attribute
    fn get_bound(&self) -> Option<String> {
        for attr in &self.attrs {
            if !attr.path().is_ident("debug") {
                continue;
            }
            // Handle #[debug(bound = "...")] format (MetaList)
            let Meta::List(list) = &attr.meta else {
                continue;
            };
            let Ok(nested) = list.parse_args_with(
                syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            for meta in nested {
                let Meta::NameValue(nv) = &meta else {
                    continue;
                };
                if !nv.path.is_ident("bound") {
                    continue;
                }
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    return Some(lit_str.value());
                }
            }
        }
        None
    }
}

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match DebugInput::from_derive_input(&input) {
        Ok(parsed) => derive_debug_impl(parsed).into(),
        Err(e) => e.write_errors().into(),
    }
}

fn derive_debug_impl(input: DebugInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let name_str = name.to_string();
    let custom_bound = input.get_bound();

    let fields = input
        .data
        .take_struct()
        .expect("Only structs are supported")
        .fields;

    // Collect type parameters
    let type_params: Vec<&Ident> = input.generics.type_params().map(|p| &p.ident).collect();

    // Track which type parameters are ONLY used in PhantomData
    let mut phantom_only_params: HashSet<String> =
        type_params.iter().map(|p| p.to_string()).collect();

    // Track which type parameters are used via associated types
    let mut associated_type_params: HashSet<String> = HashSet::new();

    // Track associated type bounds needed
    let mut associated_type_bounds: Vec<proc_macro2::TokenStream> = Vec::new();

    // Collect field debug calls
    let mut field_debug_calls = Vec::new();

    for field in &fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_ty = &field.ty;

        // Generate field debug call
        if let Some(fmt) = field.get_format() {
            field_debug_calls.push(quote! {
                .field(#field_name_str, &::std::format_args!(#fmt, &self.#field_name))
            });
        } else {
            field_debug_calls.push(quote! {
                .field(#field_name_str, &self.#field_name)
            });
        }

        // Only infer bounds if no custom bound is specified
        if custom_bound.is_none() {
            analyze_type_for_bounds(
                field_ty,
                &type_params,
                &mut phantom_only_params,
                &mut associated_type_params,
                &mut associated_type_bounds,
            );
        }
    }

    // Build the where clause
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Build where predicates
    let mut where_predicates: Vec<WherePredicate> = Vec::new();

    // Add existing where predicates
    if let Some(wc) = where_clause {
        where_predicates.extend(wc.predicates.iter().cloned());
    }

    if let Some(bound_str) = &custom_bound {
        // Parse and add custom bound
        let bound: WherePredicate =
            syn::parse_str(bound_str).expect("failed to parse custom bound");
        where_predicates.push(bound);
    } else {
        // Add Debug bounds for type parameters that need them
        for param in type_params.iter() {
            let param_str = param.to_string();

            // Skip if only used in PhantomData
            if phantom_only_params.contains(&param_str) {
                continue;
            }

            // Skip if only used via associated types
            if associated_type_params.contains(&param_str) {
                continue;
            }

            // Otherwise, add Debug bound
            where_predicates.push(parse_quote!(#param: ::std::fmt::Debug));
        }

        // Add associated type bounds
        for bound in associated_type_bounds {
            let predicate: WherePredicate = syn::parse2(quote! { #bound: ::std::fmt::Debug })
                .expect("failed to parse associated type bound");
            where_predicates.push(predicate);
        }
    }

    // Build the where clause
    let where_clause = if where_predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_predicates),* }
    };

    quote! {
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(#name_str)
                    #(#field_debug_calls)*
                    .finish()
            }
        }
    }
}

/// Analyzes a type to determine what bounds are needed
fn analyze_type_for_bounds(
    ty: &Type,
    type_params: &[&Ident],
    phantom_only_params: &mut HashSet<String>,
    associated_type_params: &mut HashSet<String>,
    associated_type_bounds: &mut Vec<proc_macro2::TokenStream>,
) {
    let Type::Path(TypePath { qself: None, path }) = ty else {
        // Handle other type variants recursively
        match ty {
            Type::Reference(type_ref) => {
                analyze_type_for_bounds(
                    &type_ref.elem,
                    type_params,
                    phantom_only_params,
                    associated_type_params,
                    associated_type_bounds,
                );
            }
            Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    analyze_type_for_bounds(
                        elem,
                        type_params,
                        phantom_only_params,
                        associated_type_params,
                        associated_type_bounds,
                    );
                }
            }
            Type::Array(type_array) => {
                analyze_type_for_bounds(
                    &type_array.elem,
                    type_params,
                    phantom_only_params,
                    associated_type_params,
                    associated_type_bounds,
                );
            }
            Type::Slice(type_slice) => {
                analyze_type_for_bounds(
                    &type_slice.elem,
                    type_params,
                    phantom_only_params,
                    associated_type_params,
                    associated_type_bounds,
                );
            }
            _ => {}
        }
        return;
    };

    let segments = &path.segments;

    // Check if this is PhantomData<T>
    if segments.len() == 1 && segments[0].ident == "PhantomData" {
        return; // PhantomData doesn't require Debug bound
    }

    // Check if first segment is a type parameter (associated type case like T::Value)
    if segments.len() > 1 {
        let first_segment = &segments[0];
        if let Some(param) = type_params.iter().find(|p| **p == &first_segment.ident) {
            let param_str = param.to_string();
            associated_type_params.insert(param_str.clone());
            phantom_only_params.remove(&param_str);
            associated_type_bounds.push(quote! { #path });
            return;
        }
    }

    // Check if this type directly is a type parameter
    if segments.len() == 1 {
        let segment = &segments[0];
        if let Some(param) = type_params.iter().find(|p| **p == &segment.ident) {
            phantom_only_params.remove(&param.to_string());
            return;
        }
    }

    // Recurse into generic arguments (e.g., Vec<T>, Option<T>)
    for segment in segments {
        let PathArguments::AngleBracketed(args) = &segment.arguments else {
            continue;
        };
        for arg in &args.args {
            let GenericArgument::Type(inner_ty) = arg else {
                continue;
            };
            analyze_type_for_bounds(
                inner_ty,
                type_params,
                phantom_only_params,
                associated_type_params,
                associated_type_bounds,
            );
        }
    }
}
