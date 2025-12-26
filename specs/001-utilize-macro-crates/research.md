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
| debug | darling errors + manual Result | Add `#[proc_macro_error]`; use `abort!` for darling errors |
| seq | syn::Result with parse_macro_input! | Add `#[proc_macro_error]`; no custom errors needed |
| sorted | Manual `syn::Error::new()` + `to_compile_error()` | Add `#[proc_macro_error]` only; keep `to_compile_error()` (see limitations) |
| bitfield-impl | Manual `syn::Error::new()` + `to_compile_error()` | Add `#[proc_macro_error]`; use `abort!` at entry points |

### ⚠️ Important Limitations: Error Output Order

**Finding from implementation testing (2025-12-25):**

The `sorted` and `check` macros have a **unique semantic requirement**: they must emit both a compile error AND the original item to allow partial compilation to continue. This creates a specific error output order requirement.

#### The Problem with `emit_error!`

**Current implementation (works correctly):**
```rust
match sorted_impl(&item) {
    Ok(tokens) => tokens.into(),
    Err(e) => {
        let error_tokens = e.to_compile_error();  // Error BEFORE item
        quote! { #error_tokens #item_tokens }.into()
    }
}
```

**Attempted migration to `emit_error!` (FAILED):**
```rust
if let Err(e) = sorted_impl(&item) {
    emit_error!(e.span(), "{}", e);  // Error accumulated, output AFTER
}
quote! { #item }.into()
```

#### Why It Fails

| Aspect | `to_compile_error()` | `emit_error!` |
|--------|---------------------|---------------|
| Error position | **Before** item in output | **After** item is processed |
| Compiler behavior | May halt early at error | Continues processing, triggers secondary errors |
| Test compatibility | ✅ Exact error output | ❌ Additional warnings/errors appear |

**Concrete test failures:**
- `04-variants-with-data.rs`: Showed 5 additional "unused import" warnings
- `05-match-expr.rs`: Showed additional "not all trait items implemented" error  
- `06-pattern-path.rs`: Showed additional "not all trait items implemented" error

#### Conclusion

For macros that must emit **error + original item together**, the manual `to_compile_error()` pattern is **semantically required**. The `#[proc_macro_error]` attribute still provides value for:
1. Consistent pattern across all macros
2. Better panic handling (converts panics to compile errors)
3. Future extensibility if requirements change

### ✅ Rationale: Keep `#[proc_macro_error]` Even Without `abort!`/`emit_error!`

**Decision (2025-12-25)**: The `#[proc_macro_error]` attribute should remain on ALL entry points, even in projects that don't use `abort!` or `emit_error!`.

#### The Panic Safety Benefit

The attribute provides **panic-to-compile-error conversion**, which is valuable on its own:

```rust
// WITHOUT #[proc_macro_error] - if panic occurs:
error: proc macro panicked
  --> src/main.rs:5:10
   |
5  | #[sorted]
   | ^^^^^^^^^
   |
   = help: message: called `Option::unwrap()` on a `None` value

// WITH #[proc_macro_error] - if panic occurs:
error: called `Option::unwrap()` on a `None` value
  --> src/main.rs:5:10
   |
5  | #[sorted]
   | ^^^^^^^^^
```

The wrapped error message is cleaner and points to the actual macro invocation.

#### Projects That Benefit

| Project | Potential Panic Sources | Panic Safety Value |
|---------|------------------------|-------------------|
| builder | `.expect()` on darling struct extraction | ✅ High |
| seq | `.expect()` in parse implementation, range handling | ✅ High |
| sorted | Pattern matching edge cases | ✅ Medium |

#### Cost-Benefit Analysis

| Factor | Keep Attribute | Remove Attribute |
|--------|---------------|------------------|
| Compile time | +~1ms per macro | Baseline |
| Binary size | No change (already in deps) | No change |
| Panic handling | ✅ Better errors | ❌ Cryptic errors |
| Consistency | ✅ All macros same pattern | ❌ Mixed patterns |
| Future changes | ✅ Can add abort! easily | ❌ Requires refactoring |

#### Final Decision

**Keep `#[proc_macro_error]` on all 7 entry points.** The panic safety benefit alone justifies the minimal overhead, and it establishes a consistent, professional pattern across the codebase.

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
| builder | ✅ Fully utilized | ✅ Added (`#[proc_macro_error]` only) | ✅ Using `ToUpperCamelCase` |
| debug | ✅ Enhanced with bound field | ✅ Added (`#[proc_macro_error]` + `abort!`) | ❌ Skip (no use case) |
| seq | ❌ Skip (not beneficial for function-like) | ✅ Added (`#[proc_macro_error]` only) | ❌ Skip (no use case) |
| sorted | ❌ Skip (minimal attribute parsing) | ✅ Added (`#[proc_macro_error]` only, see limitations) | ❌ Skip (no use case) |
| bitfield-impl | ⚠️ `#[bits = N]` deferred (syntax limitation) | ✅ Added (`#[proc_macro_error]` + `abort!`) | ✅ Using `ToSnakeCase` |

### Implementation Notes

- **builder**: `#[proc_macro_error]` present but darling's `write_errors()` handles errors (appropriate for darling integration)
- **debug**: Full `abort!` usage for darling parse errors
- **seq**: `#[proc_macro_error]` present; `parse_macro_input!` handles parse errors internally
- **sorted**: `#[proc_macro_error]` present but `to_compile_error()` preserved (see Section 2 limitations)
- **bitfield-impl**: Full `abort!` usage at entry points; internal functions use Result pattern

---

## 5. Code Reduction Results

### Initial Estimates vs Actual Results

| Project | Initial Estimate | Actual Result | Notes |
|---------|-----------------|---------------|-------|
| builder | ~33% reduction | ✅ Minor (~5%) | heck added for naming; darling already optimal |
| debug | ~46% reduction | ✅ ~35% | `get_bound()` eliminated via darling field |
| seq | ~12% reduction | ✅ ~2% | Only `#[proc_macro_error]` added |
| sorted | ~50% reduction | ❌ ~0% | `emit_error!` migration failed (see Section 2) |
| bitfield-impl | ~50% reduction | ⚠️ ~15% | `#[bits = N]` darling migration deferred |

### Why Estimates Differed from Actual

1. **sorted**: The 50% estimate assumed `emit_error!` could replace `to_compile_error()`. Testing revealed this causes secondary compiler errors due to error output ordering.

2. **bitfield-impl**: The `#[bits = N]` syntax uses `name = value` attribute format which darling's `attributes()` doesn't directly support without custom `FromMeta` implementation.

3. **builder**: Already well-optimized with darling; heck addition was cosmetic.

### Actual Reduction Summary

| Metric | Value |
|--------|-------|
| Debug project | ~23 lines removed (`get_bound()` → darling field) |
| Entry points with `#[proc_macro_error]` | 7/7 (100%) |
| Projects using `abort!` | 2/5 (debug, bitfield-impl) |
| Projects using heck case conversion | 2/5 (builder, bitfield-impl) |

**Overall**: The 30% reduction target was achieved in the **debug** project. Other projects achieved smaller reductions or maintained current patterns where semantic requirements prevented migration.

