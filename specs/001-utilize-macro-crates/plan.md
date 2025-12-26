# Implementation Plan: Utilize Macro Crates

**Branch**: `001-utilize-macro-crates` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/001-utilize-macro-crates/spec.md`

## Summary

Fully utilize `darling`, `proc-macro-error2`, and `heck` crates across all proc-macro projects to simplify code and establish consistent patterns. Focus on replacing manual attribute parsing with darling, error handling with proc-macro-error2, and case conversion with heck.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: syn 2.x, quote 1.x, proc-macro2 1.x, darling 0.20, proc-macro-error2 2.x, heck 0.5  
**Storage**: N/A (procedural macros)  
**Testing**: trybuild for compile-fail tests, standard cargo test  
**Target Platform**: All Rust-supported platforms (proc macros)  
**Project Type**: Workspace with 5 proc-macro crates  
**Performance Goals**: <10% compile time regression  
**Constraints**: Error messages must maintain semantic equivalence (same span, similar meaning)  
**Scale/Scope**: 5 proc-macro projects, ~500 LOC total

## Constitution Check

*GATE: Passed. Re-checked after implementation.*

| Gate | Status | Notes |
|------|--------|-------|
| Procedural macros recommended | ✅ Pass | darling, proc-macro-error2, heck are explicitly recommended |
| Code simplification focus | ✅ Pass | All changes reduce boilerplate |
| Test compatibility | ✅ Pass | All 46 trybuild tests pass |

## Project Structure

### Documentation (this feature)

```text
specs/001-utilize-macro-crates/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Crate capabilities and applicability
├── data-model.md        # Entity relationships (darling structs)
├── quickstart.md        # Integration scenarios
├── contracts/           # Before/after patterns per project
│   ├── builder-patterns.md
│   ├── debug-patterns.md
│   ├── seq-patterns.md
│   ├── sorted-patterns.md
│   └── bitfield-patterns.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Task breakdown (63 tasks)
```

### Source Code (repository root)

```text
builder/src/lib.rs       # Builder derive macro
debug/src/lib.rs         # CustomDebug derive macro
seq/src/lib.rs           # seq! function-like macro
sorted/src/lib.rs        # #[sorted] attribute macro
bitfield/impl/src/lib.rs # #[bitfield] attribute + BitfieldSpecifier derive
```

**Structure Decision**: Existing proc-macro workspace structure maintained. Changes are internal refactoring only.

## Implementation Status

### Completed Phases

| Phase | Description | Status |
|-------|-------------|--------|
| Phase 1: Setup | Add dependencies to all Cargo.toml files | ✅ Complete |
| Phase 2: Foundational | Baseline measurement, test verification | ✅ Complete |
| Phase 3: User Story 1 | Darling enhancements | ✅ Complete (with limitations) |
| Phase 4: User Story 2 | Proc-macro-error2 integration | ✅ Complete (with limitations) |
| Phase 5: User Story 3 | Heck case conversion | ✅ Complete |
| Phase 6: Polish | Final verification | ✅ Complete |

### Implementation Findings

#### proc-macro-error2 Semantic Limitation

**Discovery**: The `sorted` macro cannot use `emit_error!` to replace `to_compile_error()`.

**Reason**: Error output order matters. The `sorted` macro must emit `compile_error!` BEFORE the original item to prevent secondary compiler errors from appearing.

| Approach | Error Position | Secondary Errors |
|----------|---------------|------------------|
| `to_compile_error()` + `quote!` | Before item | ❌ Not shown |
| `emit_error!` | After item (by proc_macro_error) | ✅ Shown |

**Resolution**: `sorted` keeps `to_compile_error()` pattern; only `#[proc_macro_error]` attribute added.

See [research.md Section 2](./research.md#⚠️-important-limitations-error-output-order) for full analysis.

#### Why Keep `#[proc_macro_error]` Without `abort!`/`emit_error!`

**Question**: Should we remove `#[proc_macro_error]` from projects that don't use `abort!` or `emit_error!`?

**Decision**: **Keep the attribute on all entry points.**

**Rationale**: The attribute provides **panic-to-compile-error conversion** which is valuable even without the error macros:

| Benefit | Description |
|---------|-------------|
| Panic Safety | Converts cryptic "proc macro panicked" to readable errors with spans |
| Consistency | All 7 entry points follow same pattern |
| Future-Proof | Can add `abort!`/`emit_error!` without structural changes |

**Cost**: Negligible (~1ms compile time overhead; no binary size impact since crate is already a dependency).

See [research.md Section 2](./research.md#✅-rationale-keep-proc_macro_error-even-without-abortemit_error) for detailed analysis.

#### darling `#[bits = N]` Limitation

**Discovery**: darling's `#[darling(attributes(...))]` doesn't directly support `name = value` syntax like `#[bits = 8]`.

**Resolution**: `bitfield-impl`'s `get_bits_attribute()` manual parsing preserved. Would require custom `FromMeta` implementation for marginal benefit.

### Final Crate Utilization

| Project | darling | proc-macro-error2 | heck |
|---------|---------|-------------------|------|
| builder | ✅ Full | ✅ Attribute only | ✅ ToUpperCamelCase |
| debug | ✅ Enhanced | ✅ Full (abort!) | ❌ N/A |
| seq | ❌ N/A | ✅ Attribute only | ❌ N/A |
| sorted | ❌ N/A | ✅ Attribute only | ❌ N/A |
| bitfield-impl | ⚠️ Partial | ✅ Full (abort!) | ✅ ToSnakeCase |

### Test Results

All 46 tests pass across all projects:
- builder: 9 tests ✅
- debug: 8 tests ✅
- seq: 9 tests ✅
- sorted: 8 tests ✅
- bitfield: 12 tests ✅

## Complexity Tracking

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| sorted keeps to_compile_error() | Required | emit_error! causes secondary errors |
| bitfield keeps manual #[bits] parsing | Acceptable | darling requires custom FromMeta |
| builder keeps darling write_errors() | Appropriate | darling integration works well |

## References

- [research.md](./research.md) - Full crate capability analysis
- [contracts/](./contracts/) - Before/after patterns per project
- [tasks.md](./tasks.md) - Complete task breakdown with status
