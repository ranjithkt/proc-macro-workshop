use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Error, Fields, GenericArgument,
    Ident, Lit, Meta, PathArguments, Result, Type, TypePath, WherePredicate,
};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_debug_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_debug_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let name_str = name.to_string();

    // Only support named struct fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::new_spanned(
                    &input,
                    "CustomDebug only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(Error::new_spanned(
                &input,
                "CustomDebug only supports structs",
            ))
        }
    };

    // Check for #[debug(bound = "...")] attribute on the struct
    let custom_bound = get_debug_bound(&input.attrs)?;

    // Collect field information
    let mut field_debug_calls = Vec::new();

    // Collect type parameters
    let type_params: Vec<&Ident> = input.generics.type_params().map(|p| &p.ident).collect();

    // Track which type parameters are ONLY used in PhantomData (so we don't need Debug)
    let mut phantom_only_params: HashSet<String> =
        type_params.iter().map(|p| p.to_string()).collect();

    // Track which type parameters are used via associated types
    let mut associated_type_params: HashSet<String> = HashSet::new();

    // Track associated type bounds needed
    let mut associated_type_bounds: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_ty = &field.ty;

        // Check for #[debug = "format"] attribute
        let custom_format = get_debug_format(&field.attrs)?;

        if let Some(fmt) = custom_format {
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
            // Analyze the field type to determine bound requirements
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
        let bound: WherePredicate = syn::parse_str(bound_str)
            .map_err(|e| Error::new_spanned(&input, format!("failed to parse bound: {}", e)))?;
        where_predicates.push(bound);
    } else {
        // Add Debug bounds for type parameters that need them
        // A type parameter needs Debug bound if:
        // 1. It's used directly in a field (not via associated type)
        // 2. It's not only used in PhantomData
        for param in type_params.iter() {
            let param_str = param.to_string();

            // If the param is only used in PhantomData, skip it
            if phantom_only_params.contains(&param_str) {
                continue;
            }

            // If the param is only used via associated types, skip it
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

    let expanded = quote! {
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(#name_str)
                    #(#field_debug_calls)*
                    .finish()
            }
        }
    };

    Ok(expanded)
}

/// Analyzes a type to determine what bounds are needed
fn analyze_type_for_bounds(
    ty: &Type,
    type_params: &[&Ident],
    phantom_only_params: &mut HashSet<String>,
    associated_type_params: &mut HashSet<String>,
    associated_type_bounds: &mut Vec<proc_macro2::TokenStream>,
) {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            let segments = &path.segments;

            // Check if this is PhantomData<T>
            if segments.len() == 1 && segments[0].ident == "PhantomData" {
                // This is PhantomData, don't remove params from phantom_only_params
                // They stay as "phantom only" unless used elsewhere
                return;
            }

            // Check if first segment is a type parameter (associated type case)
            if segments.len() > 1 {
                let first_segment = &segments[0];
                if let Some(param) = type_params.iter().find(|p| **p == &first_segment.ident) {
                    // This is an associated type like T::Value
                    let param_str = param.to_string();

                    // Mark this param as used via associated type
                    associated_type_params.insert(param_str.clone());

                    // Remove from phantom_only since it's used here
                    phantom_only_params.remove(&param_str);

                    // Add the associated type to bounds
                    associated_type_bounds.push(quote! { #path });
                    return;
                }
            }

            // Check if this type directly contains a type parameter
            if segments.len() == 1 {
                let segment = &segments[0];
                if let Some(param) = type_params.iter().find(|p| **p == &segment.ident) {
                    // Direct use of type parameter like `T` or `value: T`
                    let param_str = param.to_string();
                    phantom_only_params.remove(&param_str);
                    return;
                }
            }

            // Recurse into generic arguments (e.g., Vec<T>, Option<T>)
            for segment in segments {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(inner_ty) = arg {
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
            }
        }
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
}

fn get_debug_format(attrs: &[Attribute]) -> Result<Option<String>> {
    for attr in attrs {
        if !attr.path().is_ident("debug") {
            continue;
        }

        // Handle #[debug = "..."] format
        if let Meta::NameValue(nv) = &attr.meta {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) = &nv.value
            {
                return Ok(Some(lit_str.value()));
            }
        }
    }
    Ok(None)
}

fn get_debug_bound(attrs: &[Attribute]) -> Result<Option<String>> {
    for attr in attrs {
        if !attr.path().is_ident("debug") {
            continue;
        }

        // Handle #[debug(bound = "...")] format
        if let Meta::List(list) = &attr.meta {
            let nested: syn::punctuated::Punctuated<Meta, syn::Token![,]> =
                list.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;

            for meta in nested {
                if let Meta::NameValue(nv) = &meta {
                    if nv.path.is_ident("bound") {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = &nv.value
                        {
                            return Ok(Some(lit_str.value()));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}
