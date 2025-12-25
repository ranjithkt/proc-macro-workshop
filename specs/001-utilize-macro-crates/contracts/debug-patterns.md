# Debug Project: Before/After Patterns

## Pattern 1: Bound Attribute Parsing with FromMeta

### Before (~25 lines)
```rust
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), forward_attrs(debug))]
struct DebugInput {
    ident: Ident,
    generics: syn::Generics,
    data: Data<(), DebugField>,
    attrs: Vec<Attribute>,
}

impl DebugInput {
    /// Get the struct-level bound from #[debug(bound = "...")] attribute
    fn get_bound(&self) -> Option<String> {
        for attr in &self.attrs {
            if !attr.path().is_ident("debug") {
                continue;
            }
            let Meta::List(list) = &attr.meta else {
                continue;
            };
            let Ok(nested) = list.parse_args_with(
                syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            for meta in nested {
                let Meta::NameValue(nv) = &meta else {
                    continue;
                };
                if !nv.path.is_ident("bound") {
                    continue;
                }
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    return Some(lit_str.value());
                }
            }
        }
        None
    }
}
```

### After (~5 lines)
```rust
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), attributes(debug))]
struct DebugInput {
    ident: Ident,
    generics: syn::Generics,
    data: Data<(), DebugField>,
    #[darling(default)]
    bound: Option<String>,  // Darling parses #[debug(bound = "...")] automatically
}

// Usage: input.bound instead of input.get_bound()
```

**Lines saved**: ~20 lines  
**Benefit**: Declarative parsing, automatic error messages with spans

---

## Pattern 2: Entry Point Error Handling

### Before
```rust
#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match DebugInput::from_derive_input(&input) {
        Ok(parsed) => derive_debug_impl(parsed).into(),
        Err(e) => e.write_errors().into(),
    }
}
```

### After
```rust
use proc_macro_error2::{abort, proc_macro_error};

#[proc_macro_derive(CustomDebug, attributes(debug))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let parsed = DebugInput::from_derive_input(&input)
        .unwrap_or_else(|e| abort!(e.span(), "{}", e));
    derive_debug_impl(parsed).into()
}
```

**Lines saved**: ~3 lines  
**Benefit**: Consistent error handling pattern

---

## Pattern 3: Remove forward_attrs (if not needed elsewhere)

### Before
```rust
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), forward_attrs(debug))]
struct DebugInput {
    // ...
    attrs: Vec<Attribute>,  // Manual access to attributes
}
```

### After
```rust
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), attributes(debug))]
struct DebugInput {
    // ...
    // attrs field removed - darling handles #[debug(...)] directly
}
```

**Benefit**: Cleaner struct, darling handles attribute routing

---

## Dependencies Change

### Cargo.toml After
```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
proc-macro-error2 = "2"
```

