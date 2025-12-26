//! Error handling macros demonstrating proc-macro-error2.
//!
//! This crate shows before/after examples of manual vs proc-macro-error2 error handling.

use proc_macro::TokenStream;
use proc_macro_error2::{abort, emit_error, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// A derive macro that validates struct fields and demonstrates error handling.
///
/// Rules:
/// - Only named structs are supported
/// - Fields named "id" must be u64
/// - Fields named "name" must be String
/// - No tuple structs
#[proc_macro_derive(Validated)]
#[proc_macro_error]
pub fn derive_validated(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│       ERROR HANDLING DEMO                │");
    eprintln!("├──────────────────────────────────────────┤");

    // Validate it's a struct with named fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                // Fatal error - can't continue
                abort!(
                    input.ident,
                    "tuple structs are not supported";
                    help = "use a struct with named fields instead"
                );
            }
            Fields::Unit => {
                abort!(input.ident, "unit structs have no fields to validate");
            }
        },
        Data::Enum(_) => {
            abort!(input.ident, "enums are not supported, only structs");
        }
        Data::Union(_) => {
            abort!(input.ident, "unions are not supported, only structs");
        }
    };

    eprintln!("│ Validating {} fields...                  │", fields.len());

    // Validate each field - accumulate errors
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        // Get type as string for simple checking
        let ty = &field.ty;
        let type_str = quote!(#ty).to_string().replace(' ', "");

        eprintln!("│   {} : {}               │", field_name_str, type_str);

        // Validation rules with error accumulation
        if field_name_str == "id" && type_str != "u64" {
            emit_error!(
                field.ty,
                "field 'id' must be of type u64";
                help = "change type to u64"
            );
        }

        if field_name_str == "name" && type_str != "String" {
            emit_error!(
                field.ty,
                "field 'name' must be of type String";
                help = "change type to String"
            );
        }

        // Check for Option<Option<T>> which is usually a bug
        if type_str.contains("Option<Option<") {
            emit_error!(
                field.ty,
                "nested Option<Option<T>> is likely a bug";
                note = "consider using Option<T> instead"
            );
        }
    }

    eprintln!("│ Validation complete!                     │");
    eprintln!("└──────────────────────────────────────────┘");

    // Generate a simple impl (if no errors were emitted, this runs)
    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> bool {
                true
            }
        }
    };

    expanded.into()
}

/// A macro that demonstrates the abort! vs emit_error! difference.
///
/// This one uses only abort! - stops at first error.
#[proc_macro_derive(StrictValidated)]
#[proc_macro_error]
pub fn derive_strict_validated(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let Data::Struct(data) = &input.data else {
        abort!(input.ident, "only structs are supported");
    };

    let Fields::Named(fields) = &data.fields else {
        abort!(input.ident, "only named fields are supported");
    };

    // With abort!, we stop at first error
    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let ty = &field.ty;
        let type_str = quote!(#ty).to_string().replace(' ', "");

        if field_name_str == "id" && type_str != "u64" {
            abort!(
                field.ty,
                "field 'id' must be u64, found {}", type_str;
                help = "change the type to u64"
            );
        }
    }

    quote! {
        impl #struct_name {
            pub fn strict_validate(&self) -> bool {
                true
            }
        }
    }
    .into()
}
