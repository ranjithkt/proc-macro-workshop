use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
use syn::__private::Span;

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

    let expanded = quote! {
        pub struct #bident {
            executable: Option<String>,
            args: Option<alloc::vec::Vec<String>>,
            env: Option<alloc::vec::Vec<String>>,
            current_dir: Option<String>,
        }
        impl #bident {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }
            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
            pub fn build(&mut self) -> Result<Command, alloc::boxed::Box<dyn std::error::Error>> {
                core::result::Result::Ok(#name {
                    executable: self.executable.take().ok_or("executable not set")?,
                    args: self.args.take().ok_or("args not set")?,
                    env: self.env.take().ok_or("env not set")?,
                    current_dir: self.current_dir.take().ok_or("current_dir not set")?,
                })
            }
        }
        impl #name {
            pub fn builder() -> #bident {
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    expanded.into()
}
