use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Visibility};

// Before moving on, have the macro also generate:
//
//     pub struct CommandBuilder {
//         executable: Option<String>,
//         args: Option<Vec<String>>,
//         env: Option<Vec<String>>,
//         current_dir: Option<String>,
//     }
//
// and in the `builder` function:
//
//     impl Command {
//         pub fn builder() -> CommandBuilder {
//             CommandBuilder {
//                 executable: None,
//                 args: None,
//                 env: None,
//                 current_dir: None,
//             }
//         }
//     }
//
//

//     impl CommandBuilder {
//         fn executable(&mut self, executable: String) -> &mut Self {
//             self.executable = Some(executable);
//             self
//         }
//
//         ...
//     }

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", input);

    let name = &input.ident;
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span());
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!()
    };

    let optionized = fields.iter().map(|field| {
        let ty = field.ty.clone();
        let optional_ty = parse_quote! {
            core::option::Option<#ty>
        };
        Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: field.mutability.clone(),
            ident: field.ident.clone(),
            colon_token: field.colon_token,
            ty: optional_ty,
        }
    });

    let functions = fields.iter().map(|field| {
        let field_name = field.ident.clone();
        quote! {
           pub fn #field_name(&mut self, #field) -> &mut Self {
               self.#field_name = Some(#field_name);
               self
           }
       }
    });

    let build_function = fields.iter().map(|field| {
        let field_name = field.ident.clone();
        quote! {
           #field_name: self.#field_name.take().unwrap_or_default()
       }
    });

    let init_fields = fields.iter().map(|field| {
        let field_name = field.ident.clone();
        quote! {
           #field_name: None
       }
    });

    let expanded = quote! {
        pub struct #bident {
            #(#optionized,)*
        }
        impl #bident {
            #(#functions)*
            pub fn build(&mut self) -> Result<Command, std::boxed::Box<dyn std::error::Error>> {
                core::result::Result::Ok(#name {
                    #(#build_function, )*
                })
            }
        }
        impl #name {
            pub fn builder() -> #bident {
                #bident {
                    #(#init_fields,)*
                }
            }
        }
    };

    expanded.into()
}
