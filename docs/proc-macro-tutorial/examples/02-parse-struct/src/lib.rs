//! Struct parsing macros for the proc-macro tutorial.
//!
//! This crate demonstrates how to use syn to parse TokenStream
//! into structured types like DeriveInput.

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

/// Derives nothing but prints the structure of the input.
///
/// This macro parses the input and uses eprintln! to show
/// what syn extracted from the TokenStream.
#[proc_macro_derive(DebugParse)]
pub fn debug_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│          PARSED STRUCTURE                │");
    eprintln!("├──────────────────────────────────────────┤");
    eprintln!("│ Name: {:<32} │", input.ident);
    eprintln!(
        "│ Visibility: {:<26} │",
        format!("{:?}", input.vis).chars().take(26).collect::<String>()
    );
    eprintln!(
        "│ Type params: {:<25} │",
        input.generics.type_params().count()
    );
    eprintln!(
        "│ Lifetimes: {:<27} │",
        input.generics.lifetimes().count()
    );
    eprintln!(
        "│ Attributes: {:<26} │",
        input.attrs.len()
    );

    match &input.data {
        Data::Struct(data) => {
            eprintln!("├──────────────────────────────────────────┤");
            eprintln!("│ Type: Struct                             │");

            match &data.fields {
                Fields::Named(fields) => {
                    eprintln!(
                        "│ Fields: {} named                          │",
                        fields.named.len()
                    );
                    eprintln!("├──────────────────────────────────────────┤");
                    for field in &fields.named {
                        let name = field.ident.as_ref().unwrap();
                        let ty = type_name(&field.ty);
                        eprintln!("│   {}: {:<31} │", name, ty);
                    }
                }
                Fields::Unnamed(fields) => {
                    eprintln!(
                        "│ Fields: {} unnamed (tuple struct)        │",
                        fields.unnamed.len()
                    );
                    eprintln!("├──────────────────────────────────────────┤");
                    for (i, field) in fields.unnamed.iter().enumerate() {
                        let ty = type_name(&field.ty);
                        eprintln!("│   .{}: {:<31} │", i, ty);
                    }
                }
                Fields::Unit => {
                    eprintln!("│ Fields: unit (no fields)                 │");
                }
            }
        }
        Data::Enum(data) => {
            eprintln!("├──────────────────────────────────────────┤");
            eprintln!("│ Type: Enum                               │");
            eprintln!(
                "│ Variants: {:<28} │",
                data.variants.len()
            );
            eprintln!("├──────────────────────────────────────────┤");
            for variant in &data.variants {
                let field_count = match &variant.fields {
                    Fields::Named(f) => format!("{} named", f.named.len()),
                    Fields::Unnamed(f) => format!("{} tuple", f.unnamed.len()),
                    Fields::Unit => "unit".to_string(),
                };
                eprintln!("│   {}: {:<29} │", variant.ident, field_count);
            }
        }
        Data::Union(_) => {
            eprintln!("├──────────────────────────────────────────┤");
            eprintln!("│ Type: Union (not commonly used)          │");
        }
    }

    eprintln!("└──────────────────────────────────────────┘");

    TokenStream::new()
}

/// Get a simplified string representation of a Type.
fn type_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let segments: Vec<_> = type_path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect();
            segments.join("::")
        }
        Type::Reference(type_ref) => {
            let inner = type_name(&type_ref.elem);
            if type_ref.mutability.is_some() {
                format!("&mut {}", inner)
            } else {
                format!("&{}", inner)
            }
        }
        _ => "...".to_string(),
    }
}

/// A derive macro that lists all fields and their attributes.
///
/// Useful for understanding how attributes are attached to fields.
#[proc_macro_derive(ListFields, attributes(my_attr))]
pub fn list_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│          FIELD ATTRIBUTES                │");
    eprintln!("├──────────────────────────────────────────┤");

    if let Data::Struct(data) = &input.data {
        for field in data.fields.iter() {
            let name = field
                .ident
                .as_ref()
                .map(|i| i.to_string())
                .unwrap_or_else(|| "unnamed".to_string());

            eprintln!("│ Field: {:<32} │", name);

            if field.attrs.is_empty() {
                eprintln!("│   (no attributes)                        │");
            } else {
                for attr in &field.attrs {
                    let path = attr
                        .path()
                        .segments
                        .iter()
                        .map(|s| s.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::");

                    let meta_type = match &attr.meta {
                        syn::Meta::Path(_) => "path",
                        syn::Meta::List(_) => "list(...)",
                        syn::Meta::NameValue(_) => "name = value",
                    };

                    eprintln!("│   #[{}] -> {}         │", path, meta_type);
                }
            }
            eprintln!("├──────────────────────────────────────────┤");
        }
    }

    eprintln!("└──────────────────────────────────────────┘");

    TokenStream::new()
}

