use proc_macro::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::quote;
use syn::{parse, parse_macro_input, parse_quote, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Ident, Meta, MetaList, Path, PathArguments, PathSegment, Type, TypePath, Visibility};

#[proc_macro_derive(Builder, attributes(builder))]
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

    fn get_option_generic_inner_type(ty: &Type) -> Option<&Type> {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = ty
        {
            if let Some(last_ident) = segments.iter().last() {
                if last_ident.ident == "Option" {
                    if let PathArguments::AngleBracketed(ref arg) = last_ident.arguments {
                        if let Some(GenericArgument::Type(s)) = arg.args.first() {
                            return Some(s);
                        }
                    }
                }
            }
        }

        None
    }

    fn get_option_generic_vec_type(field: &Field) -> Option<&Type> {
        let s = field.attrs.iter().filter_map(|attr| {
            if let Meta::List(MetaList { path, tokens, .. }, .. ) = &attr.meta {
                if let Some(PathSegment {ident, ..}) = path.segments.first() {
                    if ident == "builder" {
                        while let Some(token) = tokens.next() {
                            eprintln!("{:#?}", token);
                        }
                        return Some((path, tokens));
                    }
                }
            }
            None
        });

        if let Type::Path(TypePath {
                              path: Path { segments, .. },
                              ..
                          }) = &field.ty
        {
            if let Some(last_ident) = segments.iter().last() {
                if last_ident.ident == "Vec" {
                    if let PathArguments::AngleBracketed(ref arg) = last_ident.arguments {
                        if let Some(GenericArgument::Type(s)) = arg.args.first() {
                            return Some(s);
                        }
                    }
                }
            }
        }

        None
    }

    let optionized = fields.iter().map(|field| {
        let ty = field.ty.clone();

        let optional_ty = if get_option_generic_inner_type(&ty).is_some() {
            parse_quote! {
                #ty
            }
        } else {
            parse_quote! {
                core::option::Option<#ty>
            }
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
        let field_name = &field.ident;
        let field_type = &field.ty;
        if let Some(inner_type) = get_option_generic_inner_type(field_type) {
            quote! {
                    pub fn #field_name(&mut self, #field_name: #inner_type) -> &mut Self {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
        } else {
            quote! {
                pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            }
        }
    });

    let build_function = fields.iter().map(|field| {
        let field_name = field.ident.clone();
        if get_option_generic_inner_type(&field.ty).is_some() {
            quote! {
                #field_name: self.#field_name.clone()
            }
        } else {
            quote! {
                #field_name: self.#field_name.clone().ok_or(concat!(stringify!(#field_name), " is not set"))?
            }
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
