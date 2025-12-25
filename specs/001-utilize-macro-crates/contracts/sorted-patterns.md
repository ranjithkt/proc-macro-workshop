# Sorted Project: Before/After Patterns

## Pattern 1: sorted Entry Point

### Before (~15 lines)
```rust
#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as Item);

    match sorted_impl(&item) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            // Return both the error and the original item so compilation can continue
            let item_tokens = quote! { #item };
            let error_tokens = e.to_compile_error();
            quote! {
                #error_tokens
                #item_tokens
            }
            .into()
        }
    }
}
```

### After (~8 lines)
```rust
use proc_macro_error2::{abort, emit_error, proc_macro_error};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as Item);
    sorted_impl(&item)
}
```

**Lines saved**: ~7 lines  
**Benefit**: proc_macro_error handles error + original item emission automatically

---

## Pattern 2: sorted_impl Error Handling

### Before
```rust
fn sorted_impl(item: &Item) -> Result<proc_macro2::TokenStream> {
    let Item::Enum(item_enum) = item else {
        return Err(Error::new_spanned(
            quote! { #[sorted] },
            "expected enum or match expression",
        ));
    };

    // ... validation ...

    for i in 1..variants.len() {
        if curr_name >= prev_name {
            continue;
        }
        return Err(Error::new(
            curr_name.span(),
            format!("{} should sort before {}", curr_name, should_before),
        ));
    }

    Ok(quote! { #item })
}
```

### After
```rust
fn sorted_impl(item: &Item) -> TokenStream {
    let Item::Enum(item_enum) = item else {
        abort!(item, "expected enum or match expression");
    };

    // ... validation ...

    for i in 1..variants.len() {
        if curr_name >= prev_name {
            continue;
        }
        abort!(
            curr_name.span(),
            "{} should sort before {}",
            curr_name,
            should_before
        );
    }

    quote! { #item }.into()
}
```

**Lines saved**: ~5 lines  
**Benefit**: Direct abort! instead of Result wrapping; cleaner control flow

---

## Pattern 3: check Entry Point

### Before
```rust
#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut item_fn = parse_macro_input!(input as ItemFn);

    match check_impl(&mut item_fn) {
        Ok(()) => quote! { #item_fn }.into(),
        Err(e) => {
            let item_tokens = quote! { #item_fn };
            let error_tokens = e.to_compile_error();
            quote! {
                #error_tokens
                #item_tokens
            }
            .into()
        }
    }
}
```

### After
```rust
#[proc_macro_attribute]
#[proc_macro_error]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut item_fn = parse_macro_input!(input as ItemFn);
    check_impl(&mut item_fn);
    quote! { #item_fn }.into()
}
```

**Lines saved**: ~10 lines

---

## Pattern 4: SortedChecker with emit_error!

### Before
```rust
struct SortedChecker {
    error: Option<Error>,
}

impl VisitMut for SortedChecker {
    fn visit_expr_match_mut(&mut self, expr: &mut ExprMatch) {
        // ...
        if self.error.is_none() {
            if let Err(e) = check_match_arms_sorted(&expr.arms) {
                self.error = Some(e);
            }
        }
        // ...
    }
}
```

### After
```rust
struct SortedChecker;

impl VisitMut for SortedChecker {
    fn visit_expr_match_mut(&mut self, expr: &mut ExprMatch) {
        // ...
        if let Err(e) = check_match_arms_sorted(&expr.arms) {
            emit_error!(e.span(), "{}", e);  // Non-fatal, continues checking
        }
        // ...
    }
}
```

**Benefit**: Can accumulate multiple errors; simpler struct (no Option<Error> field)

---

## Why NOT Use Darling Here

The sorted macro uses attribute macros (`#[proc_macro_attribute]`) with minimal attribute parsing:
- `#[sorted]` on enum - no values to parse
- `#[sorted]` on match - no values to parse

Darling's overhead is not justified for empty attributes.

---

## Dependencies Change

### Cargo.toml Before
```toml
[dependencies]
syn = { version = "2", features = ["full", "parsing", "visit-mut", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
```

### Cargo.toml After
```toml
[dependencies]
syn = { version = "2", features = ["full", "parsing", "visit-mut", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
proc-macro-error2 = "2"
```

