# Implementation Plan: Proc-Macro Workshop Implementations

**Branch**: `001-proc-macro-implementations` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/001-proc-macro-implementations/spec.md`

## Summary

Implement all 5 procedural macro projects in the workshop: `builder`, `debug`, `seq`, `sorted`, and `bitfield`. Each project demonstrates different proc-macro techniques, progressing from basic derive macros to complex multi-macro systems. All implementations must pass the workshop's test suite (trybuild), follow the constitution's principles for compile-time efficiency and production-grade completeness, and serve as educational reference material.

## Technical Context

**Language/Version**: Rust stable (edition 2021)  
**Primary Dependencies**:
- `syn` 2.x (syntax parsing with minimal features per project)
- `quote` 1.x (quasi-quoting for code generation)
- `proc-macro2` 1.x (TokenStream interop)
- `darling` 0.20+ (attribute parsing for builder, debug, bitfield)
- `proc-macro-error2` 2.x (multi-error handling for sorted, bitfield)
- `heck` 0.5+ (case conversion for builder)

**Storage**: N/A (procedural macros have no runtime storage)  
**Testing**: `trybuild` compile-time test harness (pre-configured in each project)  
**Target Platform**: Platform-agnostic (Rust compiler plugin)  
**Project Type**: Multi-crate Cargo workspace (5 proc-macro crates)  
**Performance Goals**: Macro expansion < 100ms for typical inputs  
**Constraints**: Zero runtime cost; all work at compile-time  
**Scale/Scope**: 5 projects, ~40 test cases total, ~2000-3000 lines of macro code

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Status |
|-----------|-------------|--------|
| I. Compile-Time Efficiency | Minimal syn features, no cloning, no expensive ops | ✅ Plan uses targeted features |
| II. Production-Grade Completeness | Handle all edge cases, generics, visibility, hygiene | ✅ Spec covers all edge cases |
| III. Ecosystem-First Dependencies | Use syn/quote/proc-macro2, darling, proc-macro-error2 | ✅ All recommended crates selected |
| IV. Test-Driven Verification | All trybuild tests must pass | ✅ SC-001 requires 100% pass |
| V. Educational Clarity | Doc comments, logical structure, descriptive names | ✅ Spec requires readability |

**Gate Status**: ✅ PASSED — All constitution principles are addressed in the plan.

## Project Structure

### Documentation (this feature)

```text
specs/001-proc-macro-implementations/
├── plan.md              # This file
├── research.md          # Phase 0 output: crate decisions, patterns
├── data-model.md        # Phase 1 output: key data structures
├── quickstart.md        # Phase 1 output: how to test implementations
├── contracts/           # Phase 1 output: macro input/output contracts
│   ├── builder.md
│   ├── debug.md
│   ├── seq.md
│   ├── sorted.md
│   └── bitfield.md
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
# Existing workshop structure (implementations go in src/lib.rs of each)
builder/
├── Cargo.toml           # Add dependencies here
├── src/
│   └── lib.rs           # derive_builder macro implementation
└── tests/
    ├── progress.rs      # Uncomment tests as implemented
    └── 01-parse.rs ... 09-redefined-prelude-types.rs

debug/
├── Cargo.toml
├── src/
│   └── lib.rs           # derive_debug macro implementation
└── tests/
    ├── progress.rs
    └── 01-parse.rs ... 08-escape-hatch.rs

seq/
├── Cargo.toml
├── src/
│   └── lib.rs           # seq! function-like macro implementation
└── tests/
    ├── progress.rs
    └── 01-parse-header.rs ... 09-interaction-with-macrorules.rs

sorted/
├── Cargo.toml
├── src/
│   └── lib.rs           # #[sorted] and #[sorted::check] attribute macros
└── tests/
    ├── progress.rs
    └── 01-parse-enum.rs ... 08-underscore.rs

bitfield/
├── Cargo.toml           # Re-exports from impl crate
├── impl/
│   ├── Cargo.toml       # Actual proc-macro crate
│   └── src/
│       └── lib.rs       # #[bitfield] and BitfieldSpecifier derive
├── src/
│   └── lib.rs           # Specifier trait, B1-B64 types
└── tests/
    ├── progress.rs
    └── 01-specifier-types.rs ... 12-accessors-edge.rs
```

**Structure Decision**: Use existing workshop structure. Each macro crate has a `src/lib.rs` for the macro entry point. The `bitfield` project uniquely has a two-crate structure (library + proc-macro impl) since proc-macro crates can only export macros.

## Complexity Tracking

> No constitution violations requiring justification. All complexity is inherent to the workshop requirements.

| Aspect | Complexity Level | Justification |
|--------|-----------------|---------------|
| 5 separate macro crates | Medium | Workshop design; each teaches different concepts |
| bitfield dual-crate structure | Medium | Required by Rust's proc-macro export rules |
| darling dependency | Low | Reduces attribute parsing complexity significantly |
| syn feature variation | Low | Minimal features per project (constitution mandate) |

## Phase 0: Research Findings

See [research.md](./research.md) for detailed decisions on:
- syn feature selection per project
- darling vs manual parsing decisions
- Error handling strategy
- Trait bound inference approach (debug)
- Token pasting implementation (seq)
- Visitor pattern usage (sorted)
- Compile-time arithmetic (bitfield)

## Phase 1: Design Artifacts

- **Data Model**: [data-model.md](./data-model.md) — Key parsed structures and generated types
- **Contracts**: [contracts/](./contracts/) — Input syntax and output code contracts per macro
- **Quickstart**: [quickstart.md](./quickstart.md) — How to build, test, and validate implementations
