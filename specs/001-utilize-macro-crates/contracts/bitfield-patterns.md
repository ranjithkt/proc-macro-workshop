# Bitfield-Impl Project: Before/After Patterns

## Pattern 1: #[bits = N] Attribute Parsing with Darling

### Before (~20 lines)
```rust
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

// Usage:
let bits_attr = get_bits_attribute(&field.attrs)?;
```

### After (~8 lines)
```rust
use darling::{FromField, util::SpannedValue};

#[derive(FromField)]
#[darling(attributes(bits))]
struct BitfieldFieldInfo {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    bits: Option<SpannedValue<usize>>,  // Parses #[bits = N] with span
}

// Usage:
let field_info = BitfieldFieldInfo::from_field(field)?;
if let Some(bits) = &field_info.bits {
    let value = bits.as_ref();
    let span = bits.span();
}
```

**Lines saved**: ~12 lines  
**Benefit**: Declarative parsing, automatic error messages, SpannedValue preserves span

---

## Pattern 2: bitfield Entry Point

### Before
```rust
#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as ItemStruct);

    match bitfield_impl(item) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
```

### After
```rust
use proc_macro_error2::{abort, proc_macro_error};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as ItemStruct);
    bitfield_impl(item)
}
```

**Lines saved**: ~5 lines

---

## Pattern 3: BitfieldSpecifier Entry Point

### Before
```rust
#[proc_macro_derive(BitfieldSpecifier)]
pub fn derive_bitfield_specifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_specifier_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
```

### After
```rust
#[proc_macro_derive(BitfieldSpecifier)]
#[proc_macro_error]
pub fn derive_bitfield_specifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_specifier_impl(input)
}
```

**Lines saved**: ~5 lines

---

## Pattern 4: Error Handling in Implementation

### Before
```rust
fn bitfield_impl(item: ItemStruct) -> Result<proc_macro2::TokenStream> {
    let Fields::Named(named_fields) = &item.fields else {
        return Err(Error::new_spanned(
            &item,
            "bitfield only supports structs with named fields",
        ));
    };
    // ...
}

fn derive_specifier_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let Data::Enum(data) = &input.data else {
        return Err(Error::new_spanned(
            &input,
            "BitfieldSpecifier only supports enums",
        ));
    };
    // ...
}
```

### After
```rust
fn bitfield_impl(item: ItemStruct) -> TokenStream {
    let Fields::Named(named_fields) = &item.fields else {
        abort!(item, "bitfield only supports structs with named fields");
    };
    // ...
}

fn derive_specifier_impl(input: DeriveInput) -> TokenStream {
    let Data::Enum(data) = &input.data else {
        abort!(input, "BitfieldSpecifier only supports enums");
    };
    // ...
}
```

---

## Pattern 5: Getter/Setter Naming with Heck

### Before
```rust
let getter_name = format_ident!("get_{}", field_name);
let setter_name = format_ident!("set_{}", field_name);
```

### After
```rust
use heck::ToSnakeCase;

let field_str = field_name.to_string().to_snake_case();
let getter_name = format_ident!("get_{}", field_str);
let setter_name = format_ident!("set_{}", field_str);
```

**Benefit**: Ensures consistent snake_case even if field has unusual naming

---

## Dependencies Change

### Cargo.toml Before
```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing", "extra-traits", "full"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
```

### Cargo.toml After
```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing", "extra-traits", "full"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
proc-macro-error2 = "2"
heck = "0.5"
```

