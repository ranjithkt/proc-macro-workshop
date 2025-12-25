# Research: Crate Capabilities and Applicability

**Feature**: 001-utilize-macro-crates  
**Date**: 2025-12-25

## 1. Darling Crate Capabilities

### Decision
Use darling for derive macro attribute parsing where it replaces manual `Meta` extraction code.

### Rationale
Darling provides declarative attribute parsing via derive macros (`FromDeriveInput`, `FromField`, `FromMeta`, `FromVariant`). It eliminates boilerplate while providing automatic error messages with proper spans.

### Key Features Applicable to This Project

| Feature | Description | Applicable Projects |
|---------|-------------|---------------------|
| `FromDeriveInput` | Parse entire derive input struct | builder, debug, bitfield-impl |
| `FromField` | Parse individual struct fields with attributes | builder, debug, bitfield-impl |
| `FromMeta` | Parse nested attribute values | debug (`#[debug(bound = "...")]`) |
| `SpannedValue<T>` | Preserve span for error reporting | debug (format strings) |
| `#[darling(default)]` | Auto-default for optional values | builder (`each` attribute) |
| `#[darling(map = "fn")]` | Transform parsed values | bitfield-impl (`#[bits = N]`) |
| `#[darling(rename = "x")]` | Attribute name mapping | All projects as needed |

### Alternatives Considered
- **Manual syn parsing**: More verbose, but offers maximum control. Rejected for projects with complex attributes.
- **synstructure**: Older, less flexible, not updated for syn 2.x. Rejected.

### Applicability Matrix

| Project | Current Darling Usage | Recommended Changes |
|---------|----------------------|---------------------|
| builder | Uses `FromDeriveInput`, `FromField` | Already good; minor cleanup possible |
| debug | Uses `FromDeriveInput`, `FromField` | Add `FromMeta` for `get_bound()` parsing |
| seq | Not applicable | Function-like macro; darling not beneficial |
| sorted | Not applicable | Minimal attribute parsing; darling overhead not justified |
| bitfield-impl | Partial (not using for `#[bits]`) | Add `FromField` for `#[bits = N]` attribute |

---

## 2. Proc-Macro-Error2 Crate Capabilities

### Decision
Use proc-macro-error2 for all proc macro entry points to simplify error handling.

### Rationale
Proc-macro-error2 provides the `#[proc_macro_error]` attribute and `abort!`/`emit_error!` macros that eliminate manual `syn::Error::to_compile_error()` patterns and enable error accumulation.

### Key Features

| Feature | Description | Use Case |
|---------|-------------|----------|
| `#[proc_macro_error]` | Attribute on entry function | All macro entry points |
| `abort!(span, "msg")` | Immediate error with span | Replace `Err(syn::Error::new(...))` |
| `abort_call_site!("msg")` | Error at call site | Fallback errors |
| `emit_error!(span, "msg")` | Non-fatal error accumulation | Multiple validation errors |
| `emit_warning!` | Compiler warning | Deprecation notices |

### Migration Pattern

**Before** (syn::Error):
```rust
fn sorted_impl(item: &Item) -> Result<TokenStream> {
    // ...
    Err(Error::new_spanned(item, "expected enum"))
}

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    match sorted_impl(&item) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let error_tokens = e.to_compile_error();
            quote! { #error_tokens #item }.into()
        }
    }
}
```

**After** (proc-macro-error2):
```rust
use proc_macro_error2::{abort, proc_macro_error};

fn sorted_impl(item: &Item) -> TokenStream {
    // ...
    abort!(item, "expected enum");
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    sorted_impl(&item).into()
}
```

### Applicability Matrix

| Project | Current Error Handling | Recommended Changes |
|---------|----------------------|---------------------|
| builder | darling errors + manual Result | Add `#[proc_macro_error]`; keep darling errors |
| debug | darling errors + manual Result | Add `#[proc_macro_error]`; keep darling errors |
| seq | syn::Result with parse_macro_input! | Add `#[proc_macro_error]`; use abort! for parse failures |
| sorted | Manual `syn::Error::new()` + `to_compile_error()` | Full migration to `abort!` |
| bitfield-impl | Manual `syn::Error::new()` + `to_compile_error()` | Full migration to `abort!` |

---

## 3. Heck Crate Capabilities

### Decision
Use heck for case conversion only where identifiers are actively transformed.

### Rationale
Heck provides standardized case conversion traits. Per clarification, add only where it provides measurable simplification.

### Key Traits

| Trait | Method | Example |
|-------|--------|---------|
| `ToSnakeCase` | `.to_snake_case()` | `"FooBar"` → `"foo_bar"` |
| `ToPascalCase` | `.to_pascal_case()` | `"foo_bar"` → `"FooBar"` |
| `ToUpperCamelCase` | `.to_upper_camel_case()` | `"foo_bar"` → `"FooBar"` |
| `ToKebabCase` | `.to_kebab_case()` | `"foo_bar"` → `"foo-bar"` |
| `ToShoutySnakeCase` | `.to_shouty_snake_case()` | `"foo_bar"` → `"FOO_BAR"` |

### Current Case Conversion Patterns

| Project | Current Pattern | Heck Applicable? |
|---------|-----------------|------------------|
| builder | `format!("{}Builder", name)` | Yes — `to_pascal_case()` for builder name |
| debug | None | No |
| seq | `format!("{}{}", ident, value)` for pasting | No — not case conversion, just concatenation |
| sorted | None | No |
| bitfield-impl | `format_ident!("get_{}", field)`, `format_ident!("set_{}", field)` | Yes — `to_snake_case()` for getter/setter |

### Applicability Matrix

| Project | Has heck Dependency | Actively Uses Heck | Recommended |
|---------|---------------------|-------------------|-------------|
| builder | ✅ Yes (unused) | ❌ No | Use for `{}Builder` naming |
| debug | ❌ No | ❌ N/A | Do not add (no case conversion needed) |
| seq | ❌ No | ❌ N/A | Do not add (identifier pasting, not conversion) |
| sorted | ❌ No | ❌ N/A | Do not add (no case conversion needed) |
| bitfield-impl | ❌ No | ❌ N/A | Add for getter/setter naming |

---

## 4. Summary: Dependencies by Project

| Project | darling | proc-macro-error2 | heck |
|---------|---------|-------------------|------|
| builder | ✅ Already present, fully utilized | ➕ Add | ✅ Already present, start using |
| debug | ✅ Already present, enhance with FromMeta | ➕ Add | ❌ Skip (no use case) |
| seq | ❌ Skip (not beneficial for function-like) | ➕ Add | ❌ Skip (no use case) |
| sorted | ❌ Skip (minimal attribute parsing) | ➕ Add | ❌ Skip (no use case) |
| bitfield-impl | ✅ Already present, enhance for #[bits] | ➕ Add | ➕ Add |

---

## 5. Code Reduction Estimates

Based on current code analysis:

| Project | Current Parsing LOC | Estimated After | Reduction |
|---------|---------------------|-----------------|-----------|
| builder | ~45 lines (type helpers) | ~30 lines | ~33% |
| debug | ~65 lines (get_bound, get_format) | ~35 lines | ~46% |
| seq | ~40 lines (parse impl) | ~35 lines | ~12% |
| sorted | ~30 lines (error handling) | ~15 lines | ~50% |
| bitfield-impl | ~50 lines (get_bits_attribute, error handling) | ~25 lines | ~50% |

**Total estimated reduction**: ~230 LOC → ~140 LOC = **~39% reduction** (exceeds 30% target)

