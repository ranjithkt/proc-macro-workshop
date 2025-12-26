# Implementation Plan: Proc-Macro Tutorial Documentation

**Branch**: `002-proc-macro-tutorial` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/002-proc-macro-tutorial/spec.md`

## Summary

Create a comprehensive, textbook-style tutorial explaining procedural macro development in Rust. The documentation covers 6 crates (proc-macro, proc-macro2, syn, quote, darling, heck) with a building-block narrative where each chapter solves problems left by the previous. Includes Mermaid diagrams, runnable code examples with eprintln debugging, and an entertaining conversational tone.

## Technical Context

**Language/Version**: Markdown with Mermaid diagrams, Rust code examples (Rust 2021 edition)  
**Primary Dependencies**: Documentation tooling (mdBook optional), Mermaid for diagrams  
**Storage**: N/A (static markdown files)  
**Testing**: Manual review + compile-test of code examples with `cargo check`/`cargo expand`  
**Target Platform**: GitHub, VS Code, any Mermaid-compatible markdown renderer  
**Project Type**: Documentation (multiple linked markdown files)  
**Performance Goals**: Each chapter readable in <20 minutes, total tutorial <2 hours  
**Constraints**: Mermaid diagrams must render in GitHub/VS Code without plugins  
**Scale/Scope**: 6 chapters + index, ~15,000 words total, 4+ diagrams, 20+ code examples

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Applies? | Status | Notes |
|-----------|----------|--------|-------|
| I. Compile-Time Efficiency | ⚪ N/A | ✅ PASS | Documentation, not code |
| II. Production-Grade Completeness | ⚪ N/A | ✅ PASS | Documentation, not code |
| III. Ecosystem-First Dependencies | 🟢 Yes | ✅ PASS | Documents recommended crates from constitution |
| IV. Test-Driven Verification | 🟡 Partial | ✅ PASS | Code examples must compile; manual verification |
| V. Educational Clarity | 🟢 Yes | ✅ PASS | Primary goal of this feature |

**Gate Status**: ✅ PASSED — Proceed to Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/002-proc-macro-tutorial/
├── plan.md              # This file
├── research.md          # Crate documentation research
├── data-model.md        # Chapter and concept structure
├── quickstart.md        # How to read/use the tutorial
├── contracts/           # Chapter specifications
│   ├── 00-index.md
│   ├── 01-tokens.md
│   ├── 02-syn.md
│   ├── 03-quote.md
│   ├── 04-darling.md
│   ├── 05-heck.md
│   └── 06-pipeline.md
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
docs/
├── proc-macro-tutorial/
│   ├── README.md            # Index with chapter links
│   ├── 01-tokens.md         # Chapter 1: TokenStream & proc-macro2
│   ├── 02-syn.md            # Chapter 2: Parsing with syn
│   ├── 03-quote.md          # Chapter 3: Code generation with quote
│   ├── 04-darling.md        # Chapter 4: Attribute parsing with darling
│   ├── 05-heck.md           # Chapter 5: Case conversion with heck
│   ├── 06-pipeline.md       # Chapter 6: Complete pipeline synthesis
│   └── examples/            # Standalone compilable examples
│       ├── 01-token-debug/
│       ├── 02-parse-struct/
│       ├── 03-generate-impl/
│       ├── 04-darling-attrs/
│       └── 05-case-convert/
```

**Structure Decision**: Documentation-first structure with chapters in `docs/proc-macro-tutorial/`. Examples are standalone Cargo projects that readers can compile and run with `cargo expand`.

## Complexity Tracking

> No constitution violations. Documentation-only feature.

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| File format | Markdown | Universal compatibility, GitHub rendering |
| Diagram format | Mermaid | Native GitHub/VS Code support, no external tools |
| Example format | Standalone Cargo projects | Readers can `cargo run` and `cargo expand` |
| Chapter count | 6 + index | One per crate + synthesis chapter |
