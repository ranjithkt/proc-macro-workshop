//! Attribute parsing macros demonstrating darling.
//!
//! This crate shows before/after examples of manual vs darling attribute parsing.

use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

// ============================================================================
// DARLING-BASED PARSING (Clean & Simple)
// ============================================================================

/// Parsed input for our ConfigDerive macro.
#[derive(FromDeriveInput)]
#[darling(attributes(config), supports(struct_named))]
struct ConfigInput {
    ident: Ident,
    data: Data<(), ConfigField>,

    /// Optional struct-level prefix for environment variables
    #[darling(default)]
    prefix: Option<String>,
}

/// Parsed field with optional config attributes.
#[derive(FromField)]
#[darling(attributes(config))]
struct ConfigField {
    ident: Option<Ident>,
    #[allow(dead_code)]
    ty: Type,

    /// Custom environment variable name
    #[darling(default)]
    env: Option<String>,

    /// Default value if env var is not set
    #[darling(default)]
    default: Option<String>,

    /// Skip this field (use struct's Default)
    #[darling(default)]
    skip: bool,
}

/// A derive macro that generates config loading from environment variables.
///
/// # Example
///
/// ```ignore
/// #[derive(ConfigDerive)]
/// #[config(prefix = "APP_")]
/// struct Config {
///     #[config(env = "DATABASE_URL")]
///     database_url: String,
///     
///     #[config(default = "8080")]
///     port: u16,
///     
///     #[config(skip)]
///     computed: String,
/// }
/// ```
#[proc_macro_derive(ConfigDerive, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Parse with darling - this is SO much simpler than manual parsing!
    let parsed = match ConfigInput::from_derive_input(&input) {
        Ok(p) => p,
        Err(e) => return e.write_errors().into(),
    };

    let struct_name = &parsed.ident;
    let prefix = parsed.prefix.as_deref().unwrap_or("");

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│       DARLING PARSED ATTRIBUTES          │");
    eprintln!("├──────────────────────────────────────────┤");
    eprintln!("│ Struct: {:<30} │", struct_name);
    eprintln!("│ Prefix: {:<30} │", prefix);

    let fields = parsed
        .data
        .as_ref()
        .take_struct()
        .expect("only structs")
        .fields;

    let field_loaders: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let name_str = name.to_string();

            eprintln!("│ Field: {:<31} │", name_str);
            eprintln!(
                "│   env: {:<31} │",
                field.env.as_deref().unwrap_or("(auto)")
            );
            eprintln!(
                "│   default: {:<27} │",
                field.default.as_deref().unwrap_or("(none)")
            );
            eprintln!(
                "│   skip: {:<30} │",
                if field.skip { "yes" } else { "no" }
            );

            if field.skip {
                quote! { #name: Default::default() }
            } else {
                let env_name = field
                    .env
                    .clone()
                    .unwrap_or_else(|| format!("{}{}", prefix, name_str.to_uppercase()));

                if let Some(default) = &field.default {
                    quote! {
                        #name: std::env::var(#env_name)
                            .unwrap_or_else(|_| #default.to_string())
                            .parse()
                            .expect(concat!("Invalid value for ", #env_name))
                    }
                } else {
                    quote! {
                        #name: std::env::var(#env_name)
                            .expect(concat!("Missing env var: ", #env_name))
                            .parse()
                            .expect(concat!("Invalid value for ", #env_name))
                    }
                }
            }
        })
        .collect();

    eprintln!("└──────────────────────────────────────────┘");

    let expanded = quote! {
        impl #struct_name {
            pub fn from_env() -> Self {
                Self {
                    #( #field_loaders ),*
                }
            }
        }
    };

    expanded.into()
}

// ============================================================================
// Manual parsing module for comparison (see src/manual.rs)
// ============================================================================

