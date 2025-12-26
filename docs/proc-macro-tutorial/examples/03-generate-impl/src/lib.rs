//! Code generation macros for the proc-macro tutorial.
//!
//! This crate demonstrates how to use quote to generate Rust code.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Generates a simple Debug implementation.
///
/// This demonstrates the core quote! patterns:
/// - Variable interpolation with #
/// - Repetition with #(...)*
/// - String interpolation
#[proc_macro_derive(SimpleDebug)]
pub fn derive_simple_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();

    let debug_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_debugs = fields.named.iter().map(|f| {
                    let fname = f.ident.as_ref().unwrap();
                    let fname_str = fname.to_string();
                    quote! {
                        .field(#fname_str, &self.#fname)
                    }
                });
                quote! { #( #field_debugs )* }
            }
            Fields::Unnamed(fields) => {
                let field_debugs = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        .field(&self.#index)
                    }
                });
                quote! { #( #field_debugs )* }
            }
            Fields::Unit => quote! {},
        },
        _ => {
            return syn::Error::new_spanned(&input.ident, "Only structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let is_tuple = matches!(&input.data, Data::Struct(d) if matches!(d.fields, Fields::Unnamed(_)));

    let expanded = if is_tuple {
        quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_tuple(#name_str)
                        #debug_fields
                        .finish()
                }
            }
        }
    } else {
        quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(#name_str)
                        #debug_fields
                        .finish()
                }
            }
        }
    };

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│          GENERATED DEBUG IMPL            │");
    eprintln!("├──────────────────────────────────────────┤");
    eprintln!("{}", expanded);
    eprintln!("└──────────────────────────────────────────┘");

    expanded.into()
}

/// Generates getter methods for all fields.
///
/// Demonstrates format_ident! for creating method names.
#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let getters = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let methods = fields.named.iter().map(|f| {
                    let fname = f.ident.as_ref().unwrap();
                    let fty = &f.ty;
                    let getter_name = format_ident!("get_{}", fname);

                    quote! {
                        /// Returns a reference to the `#fname` field.
                        pub fn #getter_name(&self) -> &#fty {
                            &self.#fname
                        }
                    }
                });
                quote! { #( #methods )* }
            }
            _ => quote! {},
        },
        _ => {
            return syn::Error::new_spanned(&input.ident, "Only named structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #name {
            #getters
        }
    };

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│          GENERATED GETTERS               │");
    eprintln!("├──────────────────────────────────────────┤");
    eprintln!("{}", expanded);
    eprintln!("└──────────────────────────────────────────┘");

    expanded.into()
}

/// Generates a builder pattern for the struct.
///
/// This is a simplified version showing quote!'s repetition features.
#[proc_macro_derive(SimpleBuilder)]
pub fn derive_simple_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(&input.ident, "Only named structs are supported")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input.ident, "Only structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Builder struct fields (all Option<T>)
    let builder_fields = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            #name: std::option::Option<#ty>
        }
    });

    // Builder setter methods
    let builder_setters = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            pub fn #name(&mut self, value: #ty) -> &mut Self {
                self.#name = std::option::Option::Some(value);
                self
            }
        }
    });

    // Build method - extract all fields
    let build_extracts = field_names.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            #name: self.#name.take().ok_or(concat!("missing field: ", #name_str))?
        }
    });

    // Default initializers (all None)
    let default_fields = field_names.iter().map(|name| {
        quote! { #name: std::option::Option::None }
    });

    let expanded = quote! {
        pub struct #builder_name {
            #( #builder_fields, )*
        }

        impl #builder_name {
            #( #builder_setters )*

            pub fn build(&mut self) -> std::result::Result<#name, &'static str> {
                std::result::Result::Ok(#name {
                    #( #build_extracts, )*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #( #default_fields, )*
                }
            }
        }
    };

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│          GENERATED BUILDER               │");
    eprintln!("├──────────────────────────────────────────┤");
    eprintln!("{}", expanded);
    eprintln!("└──────────────────────────────────────────┘");

    expanded.into()
}

