# Data Model: Refactoring Patterns

**Feature**: 001-utilize-macro-crates  
**Date**: 2025-12-25

## Overview

This document defines the code transformation patterns for each project. The "data model" for this refactoring feature is the structure of code patterns being replaced.

---

## 1. Builder Project Patterns

### Current State
- Uses darling for `FromDeriveInput` and `FromField`
- Has heck dependency but doesn't use it
- Manual Result handling in entry point

### Transformations

| Pattern | Before | After |
|---------|--------|-------|
| Entry point | `match ... { Ok/Err }` | `#[proc_macro_error]` + direct return |
| Builder naming | `format!("{}Builder", name)` | `name.to_string().to_pascal_case() + "Builder"` |

### Code Changes
```rust
// ADD: Import
use proc_macro_error2::proc_macro_error;
use heck::ToUpperCamelCase;

// CHANGE: Entry function
#[proc_macro_derive(Builder, attributes(builder))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let parsed = BuilderInput::from_derive_input(&input)
        .unwrap_or_else(|e| { abort!(e.span(), "{}", e) });
    derive_builder_impl(parsed).into()
}
```

---

## 2. Debug Project Patterns

### Current State
- Uses darling but manually parses `#[debug(bound = "...")]` in `get_bound()`
- Manual Result handling

### Transformations

| Pattern | Before | After |
|---------|--------|-------|
| Bound parsing | `get_bound()` with manual Meta iteration | `#[darling(default)]` field with `FromMeta` |
| Entry point | `match ... { Ok/Err }` | `#[proc_macro_error]` + abort! |

### Code Changes

**Add to DebugInput struct:**
```rust
#[derive(FromDeriveInput)]
#[darling(supports(struct_named), attributes(debug))]
struct DebugInput {
    ident: Ident,
    generics: syn::Generics,
    data: Data<(), DebugField>,
    #[darling(default)]
    bound: Option<String>,  // Replaces get_bound() method
}
```

**Remove:**
- `get_bound()` method (~25 lines)
- `forward_attrs(debug)` at struct level (use `attributes(debug)` instead)

---

## 3. Seq Project Patterns

### Current State
- Manual `syn::parse::Parse` implementation
- No darling/heck (appropriate for function-like macro)

### Transformations

| Pattern | Before | After |
|---------|--------|-------|
| Entry point | `parse_macro_input!` only | Add `#[proc_macro_error]` |
| Parse errors | Implicit via `?` in Parse | Explicit `abort!` for clear messages |

### Code Changes
```rust
use proc_macro_error2::{abort, proc_macro_error};

#[proc_macro]
#[proc_macro_error]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqInput);
    // ... rest unchanged
}
```

---

## 4. Sorted Project Patterns

### Current State
- Manual `syn::Error::new()` and `to_compile_error()` throughout
- Complex error recovery pattern

### Transformations

| Pattern | Before | After |
|---------|--------|-------|
| Error creation | `Err(Error::new_spanned(x, "msg"))` | `abort!(x, "msg")` |
| Error return | `e.to_compile_error().into()` | Handled by `#[proc_macro_error]` |
| Result type | `Result<TokenStream>` | Direct `TokenStream` return |

### Code Changes

**Entry functions:**
```rust
use proc_macro_error2::{abort, proc_macro_error};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let item = parse_macro_input!(input as Item);
    sorted_impl(&item)
}

fn sorted_impl(item: &Item) -> TokenStream {
    let Item::Enum(item_enum) = item else {
        abort!(item, "expected enum or match expression");
    };
    // ... validation with abort! instead of Err(Error::new(...))
    quote! { #item }.into()
}
```

---

## 5. Bitfield-Impl Project Patterns

### Current State
- Uses darling but has manual `get_bits_attribute()` function
- Manual `syn::Error` handling

### Transformations

| Pattern | Before | After |
|---------|--------|-------|
| `#[bits = N]` parsing | `get_bits_attribute()` (~20 lines) | `FromField` with `SpannedValue<usize>` |
| Entry points | `match ... { Ok/Err }` | `#[proc_macro_error]` + abort! |
| Getter/setter naming | `format_ident!("get_{}")` | Use heck for consistency |

### Code Changes

**Field struct with darling:**
```rust
use darling::{FromField, util::SpannedValue};

#[derive(FromField)]
#[darling(attributes(bits))]
struct BitfieldField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    bits: Option<SpannedValue<usize>>,  // Replaces get_bits_attribute()
}
```

**Getter/setter with heck:**
```rust
use heck::ToSnakeCase;

let getter_name = format_ident!("get_{}", field_name.to_string().to_snake_case());
let setter_name = format_ident!("set_{}", field_name.to_string().to_snake_case());
```

---

## Entity Summary

| Entity | Fields | Relationships |
|--------|--------|---------------|
| `BuilderInput` | ident, data | Contains `Vec<BuilderField>` |
| `BuilderField` | ident, ty, each | Part of BuilderInput |
| `DebugInput` | ident, generics, data, bound | Contains `Vec<DebugField>` |
| `DebugField` | ident, ty, format | Part of DebugInput |
| `SeqInput` | var, start, end, inclusive, body | Standalone |
| `BitfieldField` | ident, ty, bits | Part of bitfield struct |

---

## Validation Rules

1. **darling parsing**: All `FromDeriveInput`/`FromField` implementations must preserve existing attribute names
2. **Error spans**: `abort!` must use the same span as current `Error::new()` calls
3. **Optional attributes**: Must use `#[darling(default)]` for optional values
4. **Type preservation**: `SpannedValue<T>` preserves span for error reporting

