//! Manual error handling for comparison with proc-macro-error2.
//!
//! This module shows how verbose error handling is WITHOUT proc-macro-error2.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Manual validation without proc-macro-error2.
///
/// Notice how every error needs .to_compile_error().into() and
/// we can only return ONE error at a time!
pub fn validate_manually(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let struct_name = &input.ident;

    // Check it's a struct
    let data = match &input.data {
        Data::Struct(data) => data,
        Data::Enum(_) => {
            return Err(syn::Error::new_spanned(&input.ident, "only structs are supported")
                .to_compile_error());
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(&input.ident, "unions not supported")
                .to_compile_error());
        }
    };

    // Check it has named fields
    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(_) => {
            return Err(
                syn::Error::new_spanned(&input.ident, "tuple structs not supported")
                    .to_compile_error(),
            );
        }
        Fields::Unit => {
            return Err(
                syn::Error::new_spanned(&input.ident, "unit structs not supported")
                    .to_compile_error(),
            );
        }
    };

    // Validate fields - but we can only return the FIRST error!
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let type_str = quote!(#field.ty).to_string().replace(' ', "");

        if field_name_str == "id" && type_str != "u64" {
            // We have to stop here! Can't accumulate errors!
            return Err(syn::Error::new_spanned(&field.ty, "field 'id' must be u64")
                .to_compile_error());
        }

        if field_name_str == "name" && type_str != "String" {
            return Err(syn::Error::new_spanned(&field.ty, "field 'name' must be String")
                .to_compile_error());
        }
    }

    // Generate output
    Ok(quote! {
        impl #struct_name {
            pub fn validate(&self) -> bool {
                true
            }
        }
    })
}

// Compare to the proc-macro-error2 version:
//
// With proc-macro-error2:
// - No explicit .to_compile_error() calls
// - Can emit_error!() to accumulate multiple errors
// - Much cleaner control flow
// - abort!() reads like a panic!() but does the right thing

