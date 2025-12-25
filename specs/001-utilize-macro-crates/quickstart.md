# Quickstart: Implementation Guide

**Feature**: 001-utilize-macro-crates  
**Date**: 2025-12-25

## Prerequisites

- Rust stable (edition 2021)
- Working `cargo test` in each project directory
- Familiarity with existing proc-macro implementations

## Implementation Order

Execute in this order to minimize integration issues:

1. **sorted** — Simplest migration (proc-macro-error2 only)
2. **seq** — Simple migration (proc-macro-error2 only)
3. **builder** — Moderate (proc-macro-error2 + activate heck)
4. **debug** — Moderate (proc-macro-error2 + darling FromMeta)
5. **bitfield-impl** — Complex (all three crates enhanced)

---

## Step-by-Step: Sorted Project

### 1. Add dependency

```bash
cd sorted
```

Edit `Cargo.toml`:
```toml
[dependencies]
proc-macro-error2 = "2"
```

### 2. Refactor lib.rs

```rust
// ADD imports
use proc_macro_error2::{abort, proc_macro_error};

// CHANGE: sorted entry point
#[proc_macro_attribute]
#[proc_macro_error]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as Item);
    sorted_impl(&item)
}

// CHANGE: sorted_impl return type and error handling
fn sorted_impl(item: &Item) -> TokenStream {
    let Item::Enum(item_enum) = item else {
        abort!(
            item,
            "expected enum or match expression"
        );
    };
    // ... rest of validation ...
    // CHANGE: abort! instead of Err(Error::new(...))
    quote! { #item }.into()
}

// CHANGE: check entry point
#[proc_macro_attribute]
#[proc_macro_error]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut item_fn = parse_macro_input!(input as ItemFn);
    check_impl(&mut item_fn);
    quote! { #item_fn }.into()
}

// CHANGE: check_impl uses emit_error! for accumulation
fn check_impl(item_fn: &mut ItemFn) {
    let mut visitor = SortedChecker;
    visitor.visit_item_fn_mut(item_fn);
}
```

### 3. Verify

```bash
cargo test
cargo clippy --all-targets
```

---

## Step-by-Step: Seq Project

### 1. Add dependency

```bash
cd seq
```

Edit `Cargo.toml`:
```toml
[dependencies]
proc-macro-error2 = "2"
```

### 2. Refactor lib.rs

```rust
// ADD import
use proc_macro_error2::proc_macro_error;

// CHANGE: entry point
#[proc_macro]
#[proc_macro_error]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqInput);
    // ... rest unchanged
}
```

### 3. Verify

```bash
cargo test
```

---

## Step-by-Step: Builder Project

### 1. Update Cargo.toml

```bash
cd builder
```

```toml
[dependencies]
proc-macro-error2 = "2"
# heck already present
```

### 2. Refactor lib.rs

```rust
// ADD imports
use proc_macro_error2::{abort, proc_macro_error};
use heck::ToUpperCamelCase;

// CHANGE: entry point
#[proc_macro_derive(Builder, attributes(builder))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match BuilderInput::from_derive_input(&input) {
        Ok(parsed) => derive_builder_impl(parsed).into(),
        Err(e) => abort!(e.span(), "{}", e),
    }
}

// OPTIONAL: Use heck for builder name
fn derive_builder_impl(input: BuilderInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let builder_name = Ident::new(
        &format!("{}Builder", name),  // Already clear, heck optional here
        name.span()
    );
    // ... rest unchanged
}
```

### 3. Verify

```bash
cargo test
```

---

## Step-by-Step: Debug Project

### 1. Update Cargo.toml

```bash
cd debug
```

```toml
[dependencies]
proc-macro-error2 = "2"
# darling already present
```

### 2. Refactor lib.rs

```rust
// ADD import
use proc_macro_error2::{abort, proc_macro_error};

// CHANGE: DebugInput to parse bound directly
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), attributes(debug))]
struct DebugInput {
    ident: Ident,
    generics: syn::Generics,
    data: Data<(), DebugField>,
    #[darling(default)]
    bound: Option<String>,  // NEW: replaces get_bound() method
}

// REMOVE: get_bound() method entirely (~25 lines)

// CHANGE: entry point
#[proc_macro_derive(CustomDebug, attributes(debug))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match DebugInput::from_derive_input(&input) {
        Ok(parsed) => derive_debug_impl(parsed).into(),
        Err(e) => abort!(e.span(), "{}", e),
    }
}

// CHANGE: Use input.bound directly instead of input.get_bound()
fn derive_debug_impl(input: DebugInput) -> proc_macro2::TokenStream {
    let custom_bound = input.bound;  // Direct field access
    // ... rest unchanged
}
```

### 3. Verify

```bash
cargo test
```

---

## Step-by-Step: Bitfield-Impl Project

### 1. Update Cargo.toml

```bash
cd bitfield/impl
```

```toml
[dependencies]
proc-macro-error2 = "2"
heck = "0.5"
# darling already present
```

### 2. Refactor lib.rs

```rust
// ADD imports
use proc_macro_error2::{abort, proc_macro_error};
use heck::ToSnakeCase;
use darling::{FromField, util::SpannedValue};

// ADD: Field struct with darling
#[derive(FromField)]
#[darling(attributes(bits))]
struct BitfieldFieldInfo {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default, rename = "bits")]
    bits_attr: Option<SpannedValue<usize>>,
}

// REMOVE: get_bits_attribute() function (~20 lines)

// CHANGE: bitfield entry point
#[proc_macro_attribute]
#[proc_macro_error]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as ItemStruct);
    bitfield_impl(item)
}

fn bitfield_impl(item: ItemStruct) -> TokenStream {
    // ... use BitfieldFieldInfo::from_field() instead of get_bits_attribute()
}

// CHANGE: derive entry point
#[proc_macro_derive(BitfieldSpecifier)]
#[proc_macro_error]
pub fn derive_bitfield_specifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_specifier_impl(input)
}
```

### 3. Verify

```bash
cd bitfield  # Parent directory
cargo test
```

---

## Verification Checklist

After all changes:

```bash
# From repo root
cargo test --all
cargo clippy --all-targets --all-features
```

### Per-Project Checks

| Project | Command | Expected |
|---------|---------|----------|
| builder | `cd builder && cargo test` | All tests pass |
| debug | `cd debug && cargo test` | All tests pass |
| seq | `cd seq && cargo test` | All tests pass |
| sorted | `cd sorted && cargo test` | All tests pass |
| bitfield | `cd bitfield && cargo test` | All tests pass |

### Success Metrics

- [ ] SC-001: Dependencies added only where beneficial
- [ ] SC-002: 30%+ reduction in parsing code (count before/after)
- [ ] SC-003: All entry points have `#[proc_macro_error]`
- [ ] SC-004: No remaining `syn::Error::new().to_compile_error()`
- [ ] SC-005: Case conversions use heck (where applicable)
- [ ] SC-006: All tests pass
- [ ] SC-007: Error spans preserved (verify with compile-fail tests)
- [ ] SC-008: No significant compile time increase

