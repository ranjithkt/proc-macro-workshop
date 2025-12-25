use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Error, Fields, GenericArgument, Ident, Lit,
    Meta, PathArguments, Result, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_builder_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_builder_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let builder_name = Ident::new(&format!("{}Builder", name), name.span());

    // Only support named struct fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::new_spanned(
                    &input,
                    "Builder only supports structs with named fields",
                ))
            }
        },
        _ => return Err(Error::new_spanned(&input, "Builder only supports structs")),
    };

    // Collect field information
    let mut builder_fields = Vec::new();
    let mut builder_inits = Vec::new();
    let mut setters = Vec::new();
    let mut build_assignments = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        // Check for #[builder(each = "...")] attribute
        let each_name = get_each_attribute(&field.attrs)?;

        // Check if the field type is Option<T>
        let is_option = is_option_type(field_ty);

        // For the builder struct, wrap in Option (unless it's already Option and no `each`)
        let builder_field_ty = if each_name.is_some() || !is_option {
            quote! { ::std::option::Option<#field_ty> }
        } else {
            // It's already Option<T>, keep as-is in builder
            quote! { #field_ty }
        };

        builder_fields.push(quote! {
            #field_name: #builder_field_ty
        });

        builder_inits.push(quote! {
            #field_name: ::std::option::Option::None
        });

        // Generate setters
        if let Some(each) = &each_name {
            let each_ident = Ident::new(each, field_name.span());

            // Get the inner type of Vec<T>
            let inner_ty = get_vec_inner_type(field_ty).ok_or_else(|| {
                Error::new_spanned(field_ty, "expected Vec<T> for #[builder(each = ...)]")
            })?;

            // Generate the single-item setter
            setters.push(quote! {
                pub fn #each_ident(&mut self, #each_ident: #inner_ty) -> &mut Self {
                    self.#field_name.get_or_insert_with(::std::vec::Vec::new).push(#each_ident);
                    self
                }
            });

            // Also generate the bulk setter if the name is different
            if each != &field_name.to_string() {
                setters.push(quote! {
                    pub fn #field_name(&mut self, #field_name: #field_ty) -> &mut Self {
                        self.#field_name = ::std::option::Option::Some(#field_name);
                        self
                    }
                });
            }
        } else if is_option {
            // For Option<T> fields, the setter takes T, not Option<T>
            let inner_ty = get_option_inner_type(field_ty).unwrap();
            setters.push(quote! {
                pub fn #field_name(&mut self, #field_name: #inner_ty) -> &mut Self {
                    self.#field_name = ::std::option::Option::Some(#field_name);
                    self
                }
            });
        } else {
            setters.push(quote! {
                pub fn #field_name(&mut self, #field_name: #field_ty) -> &mut Self {
                    self.#field_name = ::std::option::Option::Some(#field_name);
                    self
                }
            });
        }

        // Generate build assignments
        if each_name.is_some() {
            // Vec fields with `each` attribute default to empty vec
            build_assignments.push(quote! {
                #field_name: self.#field_name.clone().unwrap_or_else(::std::vec::Vec::new)
            });
        } else if is_option {
            // Option fields are just cloned as-is
            build_assignments.push(quote! {
                #field_name: self.#field_name.clone()
            });
        } else {
            // Required fields
            let err_msg = format!("{} is required", field_name);
            build_assignments.push(quote! {
                #field_name: self.#field_name.clone().ok_or(#err_msg)?
            });
        }
    }

    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_inits),*
                }
            }
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(&mut self) -> ::std::result::Result<#name, ::std::boxed::Box<dyn ::std::error::Error>> {
                ::std::result::Result::Ok(#name {
                    #(#build_assignments),*
                })
            }
        }
    };

    Ok(expanded)
}

fn get_each_attribute(attrs: &[Attribute]) -> Result<Option<String>> {
    for attr in attrs {
        if !attr.path().is_ident("builder") {
            continue;
        }

        let meta = &attr.meta;
        match meta {
            Meta::List(list) => {
                // Parse builder(each = "...")
                let nested: syn::punctuated::Punctuated<Meta, syn::Token![,]> =
                    list.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;

                for inner_meta in nested {
                    match &inner_meta {
                        Meta::NameValue(nv) if nv.path.is_ident("each") => {
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = &nv.value
                            {
                                return Ok(Some(lit_str.value()));
                            }
                        }
                        _ => {
                            // Point to the whole meta (builder(eac = "arg"))
                            return Err(Error::new_spanned(
                                meta,
                                "expected `builder(each = \"...\")`",
                            ));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::new_spanned(
                    attr,
                    "expected `builder(each = \"...\")`",
                ));
            }
        }
    }
    Ok(None)
}

fn is_option_type(ty: &Type) -> bool {
    get_option_inner_type(ty).is_some()
}

fn get_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_none() {
            let segments = &type_path.path.segments;
            if segments.len() == 1 {
                let segment = &segments[0];
                if segment.ident == "Option" {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if args.args.len() == 1 {
                            if let GenericArgument::Type(inner) = &args.args[0] {
                                return Some(inner);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_vec_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_none() {
            let segments = &type_path.path.segments;
            if segments.len() == 1 {
                let segment = &segments[0];
                if segment.ident == "Vec" {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if args.args.len() == 1 {
                            if let GenericArgument::Type(inner) = &args.args[0] {
                                return Some(inner);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
