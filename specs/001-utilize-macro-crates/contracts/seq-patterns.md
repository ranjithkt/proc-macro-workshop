# Seq Project: Before/After Patterns

## Pattern 1: Entry Point with proc_macro_error

### Before
```rust
#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqInput);

    let end = if input.inclusive {
        input.end + 1
    } else {
        input.end
    };

    // ... expansion logic
    output.into()
}
```

### After
```rust
use proc_macro_error2::proc_macro_error;

#[proc_macro]
#[proc_macro_error]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqInput);

    let end = if input.inclusive {
        input.end + 1
    } else {
        input.end
    };

    // ... expansion logic
    output.into()
}
```

**Lines changed**: +2 (import + attribute)  
**Benefit**: Enables abort! usage for future error improvements; consistent pattern across all macros

---

## Why NOT Use Darling Here

The seq macro is a **function-like procedural macro** (`#[proc_macro]`), not a derive macro. Darling is designed for:
- `#[proc_macro_derive]` - parsing derive input structs
- Attribute parsing with `FromMeta`

For function-like macros with custom syntax (`N in 0..5 { ... }`), manual `syn::parse::Parse` implementation is more appropriate.

### Current Parse Implementation (Keep As-Is)
```rust
impl Parse for SeqInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let var: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: LitInt = input.parse()?;
        // ... custom syntax parsing
    }
}
```

This is cleaner than trying to force darling's attribute-focused API onto custom syntax.

---

## Why NOT Use Heck Here

The seq macro generates identifiers via **pasting** (concatenation), not case conversion:

```rust
// Current pattern - identifier pasting
let new_name = format!("{}{}", ident, value);  // e.g., "field" + "0" = "field0"
let new_ident = Ident::new(&new_name, ident.span());
```

Heck's case conversion (`to_snake_case`, `to_pascal_case`) is not applicable here.

---

## Dependencies Change

### Cargo.toml Before
```toml
[dependencies]
syn = { version = "2", features = ["parsing", "proc-macro", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
```

### Cargo.toml After
```toml
[dependencies]
syn = { version = "2", features = ["parsing", "proc-macro", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
proc-macro-error2 = "2"
```

