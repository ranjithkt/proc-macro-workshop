# Implementation Plan: Proc-Macro Code Quality Refactoring

**Branch**: `001-proc-macro-implementations` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)  
**Input**: Code quality analysis from `/speckit.analyze` command

## Summary

Refactor all 5 procedural macro implementations to properly utilize the crates specified in the constitution and research.md. The current implementations pass all tests but violate the constitution by not using `darling` for attribute parsing and `proc-macro-error2` for multi-error handling. This refactoring will reduce code nesting from 8 levels to 3-4 levels and improve maintainability.

## Technical Context

**Language/Version**: Rust stable (edition 2021)  
**Primary Dependencies**:
- `syn` 2.x (already in use)
- `quote` 1.x (already in use)
- `proc-macro2` 1.x (already in use)
- `darling` 0.20+ (**TO BE UTILIZED** - added but not used in builder/debug)
- `proc-macro-error2` 2.x (**TO BE UTILIZED** - added but not used in sorted)
- `heck` 0.5+ (**TO BE ADDED** - for case conversion in builder)

**Storage**: N/A (procedural macros have no runtime storage)  
**Testing**: `trybuild` compile-time test harness (existing - all 46 tests must continue to pass)  
**Target Platform**: Platform-agnostic (Rust compiler plugin)  
**Project Type**: Multi-crate Cargo workspace (5 proc-macro crates)  
**Performance Goals**: Macro expansion < 100ms for typical inputs (no regression)  
**Constraints**: Zero runtime cost; all work at compile-time  
**Scale/Scope**: Refactoring ~500 lines across 4 macro crates

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Current Status | After Refactor |
|-----------|-------------|----------------|----------------|
| I. Compile-Time Efficiency | Minimal syn features, no cloning | ✅ PASS | ✅ PASS |
| II. Production-Grade Completeness | Handle all edge cases | ✅ PASS (46/46 tests) | ✅ PASS |
| III. Ecosystem-First Dependencies | Use darling, proc-macro-error2 | ❌ **FAIL** | ✅ PASS |
| IV. Test-Driven Verification | All tests pass | ✅ PASS | ✅ PASS |
| V. Educational Clarity | Clear, idiomatic code | ❌ **FAIL** (8-level nesting) | ✅ PASS |

**Gate Status**: ❌ FAILED — Constitution violations require remediation.

## Project Structure

### Documentation (this feature)

```text
specs/001-proc-macro-implementations/
├── plan.md              # This file (refactoring plan)
├── research.md          # Existing - no changes needed
├── data-model.md        # Existing - no changes needed
├── quickstart.md        # Existing - no changes needed
├── contracts/           # Existing - no changes needed
└── tasks.md             # TO BE UPDATED with refactoring tasks
```

### Source Code (files to refactor)

```text
builder/
├── Cargo.toml           # ADD: heck = "0.5"
└── src/
    └── lib.rs           # REFACTOR: Use darling for attribute parsing

debug/
├── Cargo.toml           # No changes (darling already added)
└── src/
    └── lib.rs           # REFACTOR: Use darling for attribute parsing

sorted/
├── Cargo.toml           # No changes (proc-macro-error2 already added)
└── src/
    └── lib.rs           # REFACTOR: Use proc-macro-error2 for multi-error

bitfield/impl/
├── Cargo.toml           # No changes (darling already added)
└── src/
    └── lib.rs           # REFACTOR: Use darling for #[bits] parsing

seq/
└── src/
    └── lib.rs           # NO CHANGES (manual parsing is correct per research.md)
```

**Structure Decision**: Existing structure preserved. Only `src/lib.rs` files are refactored.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |

> No complexity justifications needed - this refactoring reduces complexity.

---

## Refactoring Scope

### Phase 1: Builder Macro Refactoring

**Goal**: Replace 90 lines of manual parsing with ~20 lines of darling definitions

**Current Issues**:
- `get_each_attribute()`: 6 levels of nesting (lines 158-200)
- `get_option_inner_type()`: 8 levels of nesting (lines 207-225)
- `get_vec_inner_type()`: 8 levels of nesting (lines 228-246)

**Solution**:

```rust
use darling::{FromDeriveInput, FromField};

#[derive(FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct BuilderInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), BuilderField>,
}

#[derive(FromField)]
#[darling(attributes(builder))]
struct BuilderField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    each: Option<String>,
}
```

**Type Detection**: Keep `is_option_type()` and `is_vec_type()` but simplify with early returns:

```rust
fn get_inner_type<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a Type> {
    let Type::Path(type_path) = ty else { return None };
    if type_path.qself.is_some() { return None; }
    
    let segment = type_path.path.segments.last()?;
    if segment.ident != wrapper { return None; }
    
    let PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    let GenericArgument::Type(inner) = args.args.first()? else { return None };
    
    Some(inner)
}
```

**Estimated Changes**: -90 lines, +30 lines = net -60 lines

---

### Phase 2: Debug Macro Refactoring

**Goal**: Replace manual attribute parsing with darling

**Current Issues**:
- `get_debug_format()`: 5 levels of nesting (lines 266-284)
- `get_debug_bound()`: 6 levels of nesting (lines 286-313)

**Solution**:

```rust
use darling::{FromDeriveInput, FromField};

#[derive(FromDeriveInput)]
#[darling(attributes(debug), supports(struct_named))]
struct DebugInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), DebugField>,
    #[darling(default)]
    bound: Option<String>,  // #[debug(bound = "...")]
}

#[derive(FromField)]
#[darling(attributes(debug))]
struct DebugField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default, rename = "debug")]
    format: Option<String>,  // #[debug = "..."]
}
```

**Estimated Changes**: -50 lines, +25 lines = net -25 lines

---

### Phase 3: Sorted Macro Refactoring

**Goal**: Use `proc-macro-error2` to emit all sorting errors at once

**Current Issues**:
- `SortedChecker.errors` collects errors but only reports the first one
- User sees one error at a time instead of all violations

**Solution**:

```rust
use proc_macro_error2::{abort, emit_error, proc_macro_error};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    // ... use emit_error! for each out-of-order variant
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    // ... use emit_error! for each out-of-order match arm
}

impl VisitMut for SortedChecker {
    fn visit_expr_match_mut(&mut self, expr: &mut ExprMatch) {
        // ... emit_error! instead of collecting into Vec
    }
}
```

**Estimated Changes**: -10 lines, +15 lines = net +5 lines (but better UX)

---

### Phase 4: Bitfield Macro Refactoring

**Goal**: Use darling for `#[bits = N]` attribute parsing

**Current Issues**:
- `get_bits_attribute()`: 4 levels of nesting (lines 173-189)

**Solution**:

```rust
#[derive(FromField)]
#[darling(attributes(bits))]
struct BitfieldField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    bits: Option<usize>,  // #[bits = N]
}
```

**Estimated Changes**: -20 lines, +10 lines = net -10 lines

---

## Validation Criteria

After refactoring, ALL of the following must pass:

1. **Test Suite**: `cargo test --workspace` (46/46 tests)
2. **Clippy**: `cargo clippy --all-targets` (0 warnings)
3. **Nesting Depth**: Maximum 4 levels in any function
4. **Crate Utilization**: 
   - `darling` used in builder, debug, bitfield
   - `proc-macro-error2` used in sorted
5. **Documentation**: Doc comments on all public macros

---

## Phase 0: Research Findings

**Reference**: [research.md](./research.md) - Already complete, no new research needed.

Key decisions that apply to this refactoring:
- darling for builder, debug, bitfield attribute parsing ✓
- proc-macro-error2 for sorted multi-error handling ✓
- Manual parsing for seq (custom syntax) ✓

---

## Phase 1: Design Artifacts

**Reference**: Existing artifacts remain valid:
- [data-model.md](./data-model.md) - No changes
- [contracts/](./contracts/) - No changes
- [quickstart.md](./quickstart.md) - No changes

The refactoring is internal implementation detail; external contracts are unchanged.

---

## Summary of Changes

| File | Lines Removed | Lines Added | Net Change |
|------|---------------|-------------|------------|
| builder/Cargo.toml | 0 | 1 | +1 |
| builder/src/lib.rs | ~90 | ~30 | -60 |
| debug/src/lib.rs | ~50 | ~25 | -25 |
| sorted/src/lib.rs | ~10 | ~15 | +5 |
| bitfield/impl/src/lib.rs | ~20 | ~10 | -10 |
| **Total** | **~170** | **~81** | **-89** |

**Outcome**: ~90 fewer lines of code with better maintainability and constitution compliance.
