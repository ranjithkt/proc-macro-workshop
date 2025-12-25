# Research: Proc-Macro Workshop Implementations

**Feature**: 001-proc-macro-implementations  
**Date**: 2025-12-25

## Executive Summary

This document captures research decisions for implementing all 5 proc-macro projects. Each section addresses a specific technical decision with rationale and alternatives considered.

---

## 1. Syn Feature Selection Per Project

### Decision

Use minimal `syn` features per project to optimize compile time:

| Project | syn Features | Rationale |
|---------|-------------|-----------|
| builder | `derive`, `parsing` | Only parses derive input (structs) |
| debug | `derive`, `parsing` | Only parses derive input (structs) |
| seq | `parsing`, `proc-macro` | Custom syntax, no derive |
| sorted | `full`, `parsing`, `visit-mut` | Needs to parse/traverse function bodies and match expressions |
| bitfield | `derive`, `parsing` | Struct + enum derive inputs |

### Rationale

- Constitution Principle I mandates minimal features
- `full` feature adds ~200ms to incremental compile; only `sorted` genuinely needs it for `ItemFn` and `ExprMatch`
- `extra-traits` (Debug impls) only for development, never in final code

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Enable `full` everywhere | Unnecessary compile-time overhead |
| Use `syn 1.x` | Constitution prohibits; would cause dependency bloat |
| Avoid syn entirely | Would require reimplementing parsing; error-prone |

---

## 2. Attribute Parsing: Darling vs Manual

### Decision

| Project | Approach | Rationale |
|---------|----------|-----------|
| builder | **darling** | Complex nested attributes `#[builder(each = "...")]` |
| debug | **darling** | Multiple attribute forms `#[debug = "..."]`, `#[debug(bound = "...")]` |
| seq | **manual** | Custom syntax; darling doesn't help |
| sorted | **manual** | Only checks presence of `#[sorted]`; no complex attributes |
| bitfield | **darling** | `#[bits = N]` attributes on fields |

### Rationale

- darling eliminates boilerplate for `Meta::NameValue` and nested parsing
- Provides automatic "did you mean?" suggestions for typos
- Constitution recommends darling for complex attributes

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Manual parsing everywhere | More code, more bugs, no "did you mean?" |
| darling everywhere | Overkill for sorted (just presence check) and seq (custom syntax) |

---

## 3. Error Handling Strategy

### Decision

| Project | Strategy | Rationale |
|---------|----------|-----------|
| builder | `syn::Error` + `to_compile_error()` | Single errors per invocation |
| debug | `syn::Error` + `to_compile_error()` | Single errors per invocation |
| seq | `syn::Error` with precise spans | Single errors, need exact token spans |
| sorted | **proc-macro-error2** | Multiple out-of-order variants reported at once |
| bitfield | **proc-macro-error2** | Multiple validation errors (bits, enum variants) |

### Rationale

- `syn::Error::to_compile_error()` is sufficient for single-error cases
- `proc-macro-error2` provides `emit_error!` for collecting multiple errors
- Constitution recommends proc-macro-error2 for multi-error scenarios

### Implementation Pattern

```rust
// Single error (builder, debug, seq)
fn derive_impl(input: TokenStream) -> Result<TokenStream, syn::Error> {
    // ...
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    derive_impl(input)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

// Multiple errors (sorted, bitfield)
#[proc_macro_error2::proc_macro_error]
#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    // Use emit_error! for each issue, then abort! if any
}
```

---

## 4. Trait Bound Inference (debug project)

### Decision

Implement a custom trait bound inference algorithm that:
1. Walks field types to find generic parameters actually used
2. Excludes `PhantomData<T>` fields from adding `T: Debug` bound
3. Respects `#[debug(bound = "...")]` escape hatch to override

### Rationale

- Test 04-type-parameter.rs requires correct bounds for `Wrapper<T>` → `T: Debug`
- Test 05-phantom-data.rs requires NOT adding bounds for phantom fields
- Test 08-escape-hatch.rs requires custom bound override

### Algorithm

```text
1. Parse all generic parameters from the struct definition
2. For each field:
   a. If field has #[debug(bound = "...")], collect custom bounds
   b. If field type is PhantomData<T>, skip (no bound for T)
   c. Otherwise, walk the type tree:
      - If type contains a generic param directly, add `Param: Debug`
      - Recurse into type arguments (Option<T>, Vec<T>, etc.)
3. If struct has #[debug(bound = "...")], use ONLY those bounds (escape hatch)
4. Deduplicate bounds and add to impl's where clause
```

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Always add `T: Debug` for all generics | Fails PhantomData test |
| Never infer, require explicit bounds | Poor ergonomics; not what workshop tests expect |
| Use synstructure's bound inference | Outdated crate; doesn't handle all cases |

---

## 5. Token Pasting Implementation (seq project)

### Decision

Implement identifier pasting by:
1. Parsing `Ident~N` sequences in token trees
2. Concatenating ident string with sequence number at expansion time
3. Creating new `Ident` with span from original identifier

### Rationale

- Test 04-paste-ident.rs requires `Cpu~N` → `Cpu0`, `Cpu1`, etc.
- The `~` is not a standard Rust operator, so we process raw TokenTrees
- Span must point to original ident for good error messages

### Implementation Pattern

```rust
fn paste_idents(tokens: TokenStream, var: &Ident, value: u64) -> TokenStream {
    let mut result = Vec::new();
    let mut iter = tokens.into_iter().peekable();
    
    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Ident(ident) if iter.peek().map(is_tilde).unwrap_or(false) => {
                // Found Ident~, check if followed by our variable
                iter.next(); // consume ~
                if let Some(TokenTree::Ident(var_ident)) = iter.peek() {
                    if var_ident == var {
                        iter.next(); // consume N
                        let pasted = format!("{}{}", ident, value);
                        result.push(Ident::new(&pasted, ident.span()).into());
                        continue;
                    }
                }
                // Not our pattern, emit as-is
                result.push(tt);
                result.push(punct('~'));
            }
            TokenTree::Group(g) => {
                // Recurse into groups
                let inner = paste_idents(g.stream(), var, value);
                result.push(Group::new(g.delimiter(), inner).into());
            }
            _ => result.push(tt),
        }
    }
    result.into_iter().collect()
}
```

---

## 6. Visitor Pattern for Match Expressions (sorted project)

### Decision

Use `syn::visit_mut::VisitMut` to traverse function bodies and find `#[sorted]` match expressions.

### Rationale

- Test 05-match-expr.rs requires `#[sorted::check]` to find nested `#[sorted]` on match expressions
- Manual recursion is error-prone; VisitMut provides systematic traversal
- Can mutate AST to strip `#[sorted]` after checking

### Implementation Pattern

```rust
use syn::visit_mut::{self, VisitMut};

struct SortedChecker {
    errors: Vec<syn::Error>,
}

impl VisitMut for SortedChecker {
    fn visit_expr_match_mut(&mut self, expr: &mut ExprMatch) {
        // Check for #[sorted] attribute
        if let Some(idx) = find_sorted_attr(&expr.attrs) {
            // Validate arm order
            if let Err(e) = check_arm_order(&expr.arms) {
                self.errors.push(e);
            }
            // Strip the attribute
            expr.attrs.remove(idx);
        }
        // Continue visiting nested expressions
        visit_mut::visit_expr_match_mut(self, expr);
    }
}
```

---

## 7. Compile-Time Arithmetic (bitfield project)

### Decision

Use Rust's const generics and associated constants for compile-time bit width validation:

1. `Specifier` trait with `const BITS: usize`
2. Use const expressions in generated code for total bit calculation
3. Static assertion via type system trick for "multiple of 8" check

### Rationale

- Test 04-multiple-of-8bits.rs requires compile-time error for invalid total
- Cannot use runtime checks; must fail at compile time
- Const generics are stable and expressive enough

### Implementation Pattern

```rust
// Specifier trait
pub trait Specifier {
    const BITS: usize;
    type Storage; // u8, u16, u32, u64
}

// Generated assertion (fails compile if not multiple of 8)
const _: () = {
    struct AssertMultipleOf8<const N: usize>;
    impl<const N: usize> AssertMultipleOf8<N> {
        const OK: () = assert!(N % 8 == 0, "total bits must be multiple of 8");
    }
    let _ = AssertMultipleOf8::<{ <A as Specifier>::BITS + <B as Specifier>::BITS + ... }>::OK;
};
```

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Runtime panic in `new()` | Not compile-time; violates workshop requirements |
| Macro counting at expansion | Fragile; doesn't compose with generic specifiers |
| typenum crate | Heavy dependency; const generics are sufficient |

---

## 8. B1-B64 Type Generation (bitfield project)

### Decision

Generate `B1` through `B64` types using the `seq!` macro (dogfooding) or inline generation in the library crate.

### Rationale

- Test 01-specifier-types.rs expects these types to exist with correct `BITS` values
- 64 nearly-identical types are perfect for macro generation
- Demonstrates the value of `seq!` (if we use it)

### Implementation Pattern

```rust
// In bitfield/src/lib.rs

/// Marker type for a 1-bit field
pub enum B1 {}
impl Specifier for B1 {
    const BITS: usize = 1;
    type Storage = u8;
}

// ... repeat for B2 through B64, or generate with seq! / build.rs
```

**Note**: Storage type selection:
- B1-B8: `u8`
- B9-B16: `u16`
- B17-B32: `u32`
- B33-B64: `u64`

---

## Summary of Decisions

| Topic | Decision | Key Crates/Features |
|-------|----------|-------------------|
| syn features | Minimal per project | `derive`, `parsing`, `full` (sorted only) |
| Attribute parsing | darling for complex, manual for simple | darling 0.20+ |
| Error handling | syn::Error + proc-macro-error2 | proc-macro-error2 for multi-error |
| Trait bounds | Custom inference with escape hatch | syn type walking |
| Token pasting | TokenTree manipulation | proc-macro2 |
| Match traversal | VisitMut pattern | syn/visit_mut |
| Compile-time checks | Const generics + type tricks | Rust 1.51+ const generics |
| B1-B64 types | Generated in lib.rs | seq! or inline |

