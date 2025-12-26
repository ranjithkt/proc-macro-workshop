//! Case conversion macros demonstrating heck.
//!
//! This crate shows how to use heck for identifier case conversion.

use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Generates methods with different case styles to demonstrate heck.
///
/// For each field, generates:
/// - get_field_name() - snake_case getter
/// - setFieldName() - camelCase setter
/// - FIELD_NAME constant - SHOUTY_SNAKE_CASE
#[proc_macro_derive(CaseDemo)]
pub fn derive_case_demo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│        CASE CONVERSION DEMO              │");
    eprintln!("├──────────────────────────────────────────┤");

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(&input.ident, "Expected named fields")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input.ident, "Expected struct")
                .to_compile_error()
                .into();
        }
    };

    let methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        let field_ty = &field.ty;

        // Show all case conversions
        let snake = field_str.to_snake_case();
        let camel = field_str.to_lower_camel_case();
        let pascal = field_str.to_pascal_case();
        let kebab = field_str.to_kebab_case();
        let shouty = field_str.to_shouty_snake_case();

        eprintln!("│ Field: {:<31} │", field_str);
        eprintln!("│   snake_case:       {:<18} │", snake);
        eprintln!("│   camelCase:        {:<18} │", camel);
        eprintln!("│   PascalCase:       {:<18} │", pascal);
        eprintln!("│   kebab-case:       {:<18} │", kebab);
        eprintln!("│   SHOUTY_SNAKE:     {:<18} │", shouty);
        eprintln!("├──────────────────────────────────────────┤");

        // Generate identifiers
        let getter = format_ident!("get_{}", snake);
        let setter = format_ident!("set{}", pascal);
        let const_name = format_ident!("{}_DEFAULT", shouty);
        let kebab_str = kebab;

        quote! {
            /// Snake_case getter
            pub fn #getter(&self) -> &#field_ty {
                &self.#field_name
            }

            /// CamelCase setter
            pub fn #setter(&mut self, value: #field_ty) {
                self.#field_name = value;
            }

            /// Get field name as kebab-case string
            pub fn #field_name as_kebab(&self) -> &'static str {
                #kebab_str
            }
        }
    });

    // Generate constants
    let constants = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let shouty = field_name.to_string().to_shouty_snake_case();
        let const_name = format_ident!("{}_FIELD", shouty);

        quote! {
            pub const #const_name: &'static str = stringify!(#field_name);
        }
    });

    eprintln!("└──────────────────────────────────────────┘");

    let expanded = quote! {
        impl #struct_name {
            #( #constants )*
            #( #methods )*
        }
    };

    expanded.into()
}

/// Generates an as_str() method for enums with kebab-case output.
#[proc_macro_derive(EnumKebab)]
pub fn derive_enum_kebab(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(&input.ident, "Expected enum")
                .to_compile_error()
                .into();
        }
    };

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│        ENUM KEBAB CONVERSION             │");
    eprintln!("├──────────────────────────────────────────┤");

    let arms = variants.iter().map(|variant| {
        let name = &variant.ident;
        let kebab = name.to_string().to_kebab_case();

        eprintln!("│ {:>20} -> {:<17} │", name, kebab);

        quote! {
            Self::#name => #kebab
        }
    });

    eprintln!("└──────────────────────────────────────────┘");

    let expanded = quote! {
        impl #enum_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #( #arms, )*
                }
            }
        }
    };

    expanded.into()
}

/// Generates a builder with PascalCase type name.
#[proc_macro_derive(BuilderNamed)]
pub fn derive_builder_named(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let struct_str = struct_name.to_string();

    // Create builder name: my_config -> MyConfigBuilder
    let builder_name = format_ident!("{}Builder", struct_str.to_pascal_case());

    eprintln!("┌──────────────────────────────────────────┐");
    eprintln!("│        BUILDER NAMING                    │");
    eprintln!("├──────────────────────────────────────────┤");
    eprintln!("│ Struct: {:<30} │", struct_str);
    eprintln!("│ Builder: {:<29} │", builder_name);
    eprintln!("└──────────────────────────────────────────┘");

    let expanded = quote! {
        pub struct #builder_name;

        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name
            }
        }
    };

    expanded.into()
}

