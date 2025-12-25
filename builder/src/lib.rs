use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericArgument, Ident, PathArguments, Type};

/// Field information parsed by darling
#[derive(FromField)]
#[darling(attributes(builder))]
struct BuilderField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    each: Option<String>,
}

/// Derive input parsed by darling
#[derive(FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct BuilderInput {
    ident: Ident,
    data: Data<(), BuilderField>,
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match BuilderInput::from_derive_input(&input) {
        Ok(parsed) => derive_builder_impl(parsed).into(),
        Err(e) => e.write_errors().into(),
    }
}

fn derive_builder_impl(input: BuilderInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let builder_name = Ident::new(&format!("{}Builder", name), name.span());

    let fields = input
        .data
        .take_struct()
        .expect("Only structs are supported")
        .fields;

    let mut builder_fields = Vec::new();
    let mut builder_inits = Vec::new();
    let mut setters = Vec::new();
    let mut build_assignments = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        let each_name = &field.each;

        // Check if the field type is Option<T>
        let is_option = is_option_type(field_ty);

        // For the builder struct, wrap in Option (unless it's already Option and no `each`)
        let builder_field_ty = if each_name.is_some() || !is_option {
            quote! { ::std::option::Option<#field_ty> }
        } else {
            quote! { #field_ty }
        };

        builder_fields.push(quote! {
            #field_name: #builder_field_ty
        });

        builder_inits.push(quote! {
            #field_name: ::std::option::Option::None
        });

        // Generate setters
        if let Some(each) = each_name {
            let each_ident = Ident::new(each, field_name.span());

            // Get the inner type of Vec<T>
            let inner_ty = get_inner_type(field_ty, "Vec")
                .expect("expected Vec<T> for #[builder(each = ...)]");

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
            let inner_ty = get_inner_type(field_ty, "Option").unwrap();
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

    quote! {
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
    }
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    get_inner_type(ty, "Option").is_some()
}

/// Extract the inner type from a wrapper type like Option<T> or Vec<T>
/// Uses early returns to reduce nesting (max 4 levels)
fn get_inner_type<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }

    let segment = type_path.path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    let GenericArgument::Type(inner) = args.args.first()? else {
        return None;
    };

    Some(inner)
}
