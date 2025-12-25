use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Error, Fields, Ident, ItemStruct, Lit, Meta,
    Result, Type,
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as ItemStruct);

    match bitfield_impl(item) {
        Ok(tokens) => tokens.into(),
        Err(e) => abort!(e.span(), "{}", e),
    }
}

// Type alias to avoid clippy::type_complexity warning
type FieldInfo<'a> = (&'a Ident, &'a Type, Option<(usize, proc_macro2::Span)>);

fn bitfield_impl(item: ItemStruct) -> Result<proc_macro2::TokenStream> {
    let name = &item.ident;
    let vis = &item.vis;

    let Fields::Named(named_fields) = &item.fields else {
        return Err(Error::new_spanned(
            &item,
            "bitfield only supports structs with named fields",
        ));
    };

    // Collect field info
    let mut field_infos: Vec<FieldInfo<'_>> = Vec::new();
    let mut total_bits_terms: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in named_fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        let bits_attr = get_bits_attribute(&field.attrs)?;

        field_infos.push((field_name, field_ty, bits_attr));
        total_bits_terms.push(quote! { <#field_ty as ::bitfield::Specifier>::BITS });
    }

    // Generate the size expression (total bits / 8)
    let size_expr = if total_bits_terms.is_empty() {
        quote! { 0 }
    } else {
        let first = &total_bits_terms[0];
        let rest = &total_bits_terms[1..];
        quote! { (#first #(+ #rest)*) / 8 }
    };

    // Generate the mod 8 check expression
    let total_bits_for_check = if total_bits_terms.is_empty() {
        quote! { 0 }
    } else {
        let first = &total_bits_terms[0];
        let rest = &total_bits_terms[1..];
        quote! { (#first #(+ #rest)*) % 8 }
    };

    // Generate getters and setters
    let mut accessors = Vec::new();
    let mut bit_offset_parts: Vec<proc_macro2::TokenStream> = Vec::new();

    for (idx, (field_name, field_ty, bits_attr)) in field_infos.iter().enumerate() {
        // Use heck for consistent snake_case naming even if field has unusual casing
        let field_str = field_name.to_string().to_snake_case();
        let getter_name = format_ident!("get_{}", field_str);
        let setter_name = format_ident!("set_{}", field_str);

        // Calculate bit offset for this field
        let current_offset = if bit_offset_parts.is_empty() {
            quote! { 0usize }
        } else {
            let parts = &bit_offset_parts;
            quote! { #(#parts)+* }
        };

        // Generate #[bits = N] check if specified
        let bits_check = bits_attr.as_ref().map_or(quote! {}, |(expected, span)| {
            let check_name = format_ident!("__bits_check_{}", idx);
            quote_spanned! {*span=>
                const #check_name: [(); #expected] = [(); <#field_ty as ::bitfield::Specifier>::BITS];
            }
        });

        accessors.push(quote! {
            #bits_check

            #vis fn #getter_name(&self) -> <#field_ty as ::bitfield::Specifier>::Bytes {
                let start_bit = #current_offset;
                let bit_count = <#field_ty as ::bitfield::Specifier>::BITS;
                let raw = self.get_bits(start_bit, bit_count);
                <#field_ty as ::bitfield::Specifier>::from_u64(raw)
            }

            #vis fn #setter_name(&mut self, value: <#field_ty as ::bitfield::Specifier>::Bytes) {
                let start_bit = #current_offset;
                let bit_count = <#field_ty as ::bitfield::Specifier>::BITS;
                let raw = <#field_ty as ::bitfield::Specifier>::into_u64(value);
                self.set_bits(start_bit, bit_count, raw);
            }
        });

        bit_offset_parts.push(quote! { <#field_ty as ::bitfield::Specifier>::BITS });
    }

    Ok(quote! {
        #[repr(C)]
        #vis struct #name {
            data: [u8; #size_expr],
        }

        impl #name {
            #vis fn new() -> Self {
                Self { data: [0; #size_expr] }
            }

            fn get_bits(&self, start: usize, count: usize) -> u64 {
                let mut result: u64 = 0;
                for i in 0..count {
                    let bit_idx = start + i;
                    let byte_idx = bit_idx / 8;
                    let bit_in_byte = bit_idx % 8;
                    if (self.data[byte_idx] >> bit_in_byte) & 1 == 1 {
                        result |= 1u64 << i;
                    }
                }
                result
            }

            fn set_bits(&mut self, start: usize, count: usize, value: u64) {
                for i in 0..count {
                    let bit_idx = start + i;
                    let byte_idx = bit_idx / 8;
                    let bit_in_byte = bit_idx % 8;
                    if (value >> i) & 1 == 1 {
                        self.data[byte_idx] |= 1 << bit_in_byte;
                    } else {
                        self.data[byte_idx] &= !(1 << bit_in_byte);
                    }
                }
            }

            #(#accessors)*
        }

        // Compile-time check that total bits is multiple of 8
        impl #name {
            const __BITS_CHECK: () = {
                let _ = <
                    <::bitfield::checks::Modulo<{ #total_bits_for_check }> as ::bitfield::checks::ModuloEight>::Mod
                    as ::bitfield::checks::TotalSizeIsMultipleOfEightBits
                >::CHECK;
            };
        }
    })
}

fn get_bits_attribute(attrs: &[syn::Attribute]) -> Result<Option<(usize, proc_macro2::Span)>> {
    for attr in attrs {
        if !attr.path().is_ident("bits") {
            continue;
        }

        let Meta::NameValue(nv) = &attr.meta else {
            continue;
        };

        let syn::Expr::Lit(syn::ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) = &nv.value
        else {
            continue;
        };

        return Ok(Some((lit_int.base10_parse()?, lit_int.span())));
    }
    Ok(None)
}

#[proc_macro_derive(BitfieldSpecifier)]
#[proc_macro_error]
pub fn derive_bitfield_specifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_specifier_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(e) => abort!(e.span(), "{}", e),
    }
}

fn derive_specifier_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        return Err(Error::new_spanned(
            &input,
            "BitfieldSpecifier only supports enums",
        ));
    };

    let variants = &data.variants;
    let variant_count = variants.len();

    // Check that variant count is a power of 2
    if variant_count == 0 || (variant_count & (variant_count - 1)) != 0 {
        return Err(Error::new_spanned(
            name,
            "BitfieldSpecifier expected a number of variants which is a power of 2",
        ));
    }

    // Calculate BITS (log2 of variant count)
    let bits = (variant_count as f64).log2() as usize;

    // Determine the bytes type
    let bytes_ty: Type = match bits {
        0..=8 => parse_quote! { u8 },
        9..=16 => parse_quote! { u16 },
        17..=32 => parse_quote! { u32 },
        _ => parse_quote! { u64 },
    };

    // Generate from_bytes match arms and discriminant checks
    let mut from_arms = Vec::new();
    let mut discriminant_checks = Vec::new();
    let max_discriminant = 1usize << bits;

    for variant in variants.iter() {
        let variant_name = &variant.ident;
        let variant_span = variant_name.span();

        from_arms.push(quote! {
            x if x == #name::#variant_name as #bytes_ty => #name::#variant_name,
        });

        // Generate compile-time check that discriminant is in range
        discriminant_checks.push(quote_spanned! {variant_span=>
            const _: () = {
                let _ = <
                    <::bitfield::checks::CheckDiscriminantInRange<
                        { (#name::#variant_name as usize) < #max_discriminant }
                    > as ::bitfield::checks::GetBoolType>::Type
                    as ::bitfield::checks::DiscriminantInRange
                >::CHECK;
            };
        });
    }

    // Add a catch-all panic case
    from_arms.push(quote! {
        _ => panic!("invalid discriminant"),
    });

    Ok(quote! {
        impl ::bitfield::Specifier for #name {
            const BITS: usize = #bits;
            type Bytes = #name;

            fn from_u64(val: u64) -> Self::Bytes {
                let val = val as #bytes_ty;
                match val {
                    #(#from_arms)*
                }
            }

            fn into_u64(val: Self::Bytes) -> u64 {
                val as u64
            }
        }

        #(#discriminant_checks)*
    })
}
