# Implementation Plan: Utilize Macro Crates

**Branch**: `001-utilize-macro-crates` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-utilize-macro-crates/spec.md`

## Summary

Refactor all proc-macro projects (builder, debug, seq, sorted, bitfield-impl) to fully utilize darling, proc-macro-error2, and heck crates where they provide measurable code simplification. The goal is to reduce manual attribute parsing code by at least 30% while maintaining semantic equivalence for error messages and ensuring all existing tests pass.

## Technical Context

**Language/Version**: Rust stable (edition 2021)  
**Primary Dependencies**: syn 2.x, quote 1.x, proc-macro2 1.x, darling 0.20+, proc-macro-error2 2.x, heck 0.5+  
**Storage**: N/A (proc macros don't use storage)  
**Testing**: trybuild (compile-pass and compile-fail tests with .stderr verification)  
**Target Platform**: Rust compiler plugin (proc-macro crate type)  
**Project Type**: Multi-crate workspace (5 proc-macro crates)  
**Performance Goals**: < 100ms macro expansion per invocation; < 10% compile time increase  
**Constraints**: Semantic equivalence for error spans; existing tests must pass; value-based dependency addition  
**Scale/Scope**: 5 proc-macro crates totaling ~850 lines of parsing/error handling code

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Design Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Compile-Time Efficiency First | ✅ PASS | darling, proc-macro-error2, heck have negligible overhead per constitution |
| II. Production-Grade Completeness | ✅ PASS | Refactoring maintains all edge case handling; tests verify completeness |
| III. Ecosystem-First Dependencies | ✅ PASS | Feature explicitly adopts constitution-recommended crates |
| IV. Test-Driven Verification | ✅ PASS | All trybuild tests must pass; semantic equivalence for error messages |
| V. Educational Clarity | ✅ PASS | Declarative darling syntax improves readability over manual parsing |

**Gate Result**: PASS — No violations. Proceeded to Phase 0.

### Post-Design Re-Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Compile-Time Efficiency First | ✅ PASS | Value-based dependency addition (skip where not beneficial) |
| II. Production-Grade Completeness | ✅ PASS | Before/after patterns preserve all functionality |
| III. Ecosystem-First Dependencies | ✅ PASS | Following constitution recommendations exactly |
| IV. Test-Driven Verification | ✅ PASS | Quickstart includes verification steps per project |
| V. Educational Clarity | ✅ PASS | Contracts show clear before/after transformations |

**Gate Result**: PASS — Design validated. Ready for task breakdown.

## Project Structure

### Documentation (this feature)

```text
specs/001-utilize-macro-crates/
├── plan.md              # This file
├── research.md          # Phase 0: Crate capabilities research
├── data-model.md        # Phase 1: Refactoring patterns per project
├── quickstart.md        # Phase 1: Implementation guide
├── contracts/           # Phase 1: Before/after code patterns
│   ├── builder-patterns.md
│   ├── debug-patterns.md
│   ├── seq-patterns.md
│   ├── sorted-patterns.md
│   └── bitfield-patterns.md
└── tasks.md             # Phase 2: Task breakdown (via /speckit.tasks)
```

### Source Code (repository root)

```text
builder/
├── Cargo.toml           # Add proc-macro-error2 (darling, heck already present)
└── src/lib.rs           # Refactor: utilize heck, add #[proc_macro_error]

debug/
├── Cargo.toml           # Add proc-macro-error2 (heck not needed - no case conversion)
└── src/lib.rs           # Refactor: FromMeta for bound parsing, add #[proc_macro_error]

seq/
├── Cargo.toml           # Add proc-macro-error2 (darling/heck only if beneficial)
└── src/lib.rs           # Refactor: add #[proc_macro_error], evaluate darling fit

sorted/
├── Cargo.toml           # Add proc-macro-error2 (darling/heck only if beneficial)
└── src/lib.rs           # Refactor: replace syn::Error with abort!, add #[proc_macro_error]

bitfield/impl/
├── Cargo.toml           # Add proc-macro-error2, heck (darling already present)
└── src/lib.rs           # Refactor: FromField for #[bits = N], add #[proc_macro_error]
```

**Structure Decision**: Existing multi-crate workspace structure preserved. Changes are limited to Cargo.toml dependencies and lib.rs refactoring within each proc-macro crate.

## Complexity Tracking

> No constitution violations requiring justification.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | — | — |

---

## Phase 0: Research Complete

See [research.md](./research.md) for detailed findings on:
- darling capabilities and applicability per project
- proc-macro-error2 migration patterns
- heck trait usage for case conversion

## Phase 1: Design Complete

See:
- [data-model.md](./data-model.md) — Refactoring patterns and code transformations
- [quickstart.md](./quickstart.md) — Step-by-step implementation guide
- [contracts/](./contracts/) — Before/after code examples per project
