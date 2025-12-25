# Builder Project: Before/After Patterns

## Pattern 1: Entry Point Error Handling

### Before
```rust
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match BuilderInput::from_derive_input(&input) {
        Ok(parsed) => derive_builder_impl(parsed).into(),
        Err(e) => e.write_errors().into(),
    }
}
```

### After
```rust
use proc_macro_error2::{abort, proc_macro_error};

#[proc_macro_derive(Builder, attributes(builder))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let parsed = BuilderInput::from_derive_input(&input)
        .unwrap_or_else(|e| abort!(e.span(), "{}", e));
    derive_builder_impl(parsed).into()
}
```

**Lines saved**: ~3 lines  
**Benefit**: Cleaner control flow, consistent error handling pattern

---

## Pattern 2: Heck for Builder Naming (Optional)

### Before
```rust
let builder_name = Ident::new(&format!("{}Builder", name), name.span());
```

### After
```rust
use heck::ToUpperCamelCase;

// When name might need normalization:
let builder_name = Ident::new(
    &format!("{}Builder", name.to_string().to_upper_camel_case()),
    name.span()
);
```

**Note**: Current implementation already uses PascalCase names, so this is optional. Apply only if normalizing unusual input names.

---

## Dependencies Change

### Cargo.toml Before
```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
heck = "0.5"
```

### Cargo.toml After
```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
heck = "0.5"
proc-macro-error2 = "2"
```

