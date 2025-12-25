<!--
=============================================================================
SYNC IMPACT REPORT
=============================================================================
Version change: 1.0.0 → 1.1.0

Modified principles:
  - III. Ecosystem-First Dependencies: Expanded crate guidance with detailed
    recommendations, replaced unmaintained proc-macro-error with proc-macro-error2,
    added heck for case conversion

Added sections: None

Removed sections: None

Templates requiring updates:
  - .specify/templates/plan-template.md: ✅ No changes needed (generic)
  - .specify/templates/spec-template.md: ✅ No changes needed (generic)
  - .specify/templates/tasks-template.md: ✅ No changes needed (generic)

Follow-up TODOs: None

=============================================================================
-->

# Proc-Macro Workshop Constitution

## Core Principles

### I. Compile-Time Efficiency First

All procedural macro implementations MUST prioritize compilation speed as the primary performance metric.

**Non-negotiables:**
- MUST minimize token stream parsing overhead by using targeted parsing (parse only what is needed)
- MUST prefer `syn`'s `Parse` implementations with minimal feature flags
- MUST avoid unnecessary cloning of syntax trees; use references and borrows where possible
- MUST avoid debug-only features (e.g., `syn/extra-traits`) in release builds
- SHOULD cache parsed information when the same data is accessed multiple times
- MUST NOT perform expensive operations (regex, file I/O, network calls) during macro expansion

**Rationale:** Procedural macros execute during compilation. Slow macros compound across every crate recompilation, directly impacting developer productivity.

### II. Production-Grade Completeness

Every macro implementation MUST handle all edge cases and produce code suitable for production environments.

**Non-negotiables:**
- MUST handle all valid Rust syntax that the macro claims to support
- MUST generate correct code for generics (lifetime parameters, type parameters, const generics)
- MUST correctly handle visibility modifiers (`pub`, `pub(crate)`, `pub(super)`, etc.)
- MUST respect and preserve attributes not consumed by the macro
- MUST generate hygienic code (avoid identifier collisions with user code)
- MUST support `#[cfg(...)]` attributes on fields and variants where applicable
- Error messages MUST use `compile_error!` or Span-based diagnostics with precise source locations
- Error messages MUST be actionable (tell the user what went wrong AND how to fix it)

**Rationale:** Production codebases contain complex generic types, visibility patterns, and conditional compilation. Macros that only work on simple cases create technical debt.

### III. Ecosystem-First Dependencies

Implementations MUST leverage established crates.io libraries instead of reinventing parsing or code generation. The compile-time overhead of these helper crates is negligible compared to the development velocity and reliability they provide.

**Mandatory crates (all projects):**

| Crate | Version | Purpose | Performance Impact |
|-------|---------|---------|-------------------|
| `syn` | 2.x | Rust syntax parsing | Core dependency; use minimal features |
| `quote` | 1.x | Quasi-quoting for code generation | Negligible; zero-cost abstractions |
| `proc-macro2` | 1.x | TokenStream interop, Span manipulation | Negligible; required by syn/quote |

**Recommended crates (use when they simplify implementation):**

| Crate | Version | When to Use | Performance Impact |
|-------|---------|-------------|-------------------|
| `darling` | 0.20+ | Complex helper attribute parsing (e.g., `#[builder(each = "...")]`) | Negligible; similar to serde derive overhead |
| `proc-macro-error2` | 2.x | When needing multiple error emission or `abort!`/`emit_error!` macros | Negligible; only adds error formatting utilities |
| `heck` | 0.5+ | Case conversion (snake_case ↔ PascalCase ↔ camelCase) for generated identifiers | Negligible; pure string transformation |

**⚠️ Crates to AVOID:**
- `proc-macro-error` (original) — **Unmaintained since 2021**, depends on `syn 1.x`, creates duplicate dependency trees. Use `proc-macro-error2` instead.
- `synstructure` — Older, less flexible than manual `syn` usage with `quote`
- Any crate not updated for `syn 2.x` — Will cause dependency bloat

**When to use darling vs manual parsing:**
- **Use darling** when: Multiple helper attributes, nested attribute values, optional fields with defaults, "did you mean?" error suggestions desired
- **Use manual syn parsing** when: Simple attributes (single value), maximum control over error messages, learning purposes

**When to use proc-macro-error2 vs syn's Error:**
- **Use proc-macro-error2** when: Emitting multiple errors from one macro invocation, wanting `abort!` macro ergonomics, cleaner `?` operator usage
- **Use syn's Error** when: Simple single-error cases, avoiding additional dependencies, fine-grained span control

**Feature flag discipline:**
- MUST enable only the `syn` features actually used (e.g., `parsing`, `derive`, `full`)
- MUST NOT enable `syn/extra-traits` in production (only for debugging during development)
- MUST NOT enable `syn/full` unless parsing expressions, statements, or items beyond derive input

**Rationale:** Battle-tested libraries reduce bugs, improve compile times (shared dependencies), and represent community best practices. The overhead of these crates is dwarfed by the complexity they encapsulate.

### IV. Test-Driven Verification

All workshop tests MUST pass after implementation. The trybuild harness is the source of truth.

**Non-negotiables:**
- MUST pass ALL numbered tests (01-xx.rs) in each project's `tests/` directory
- MUST pass both compile-success AND compile-fail tests
- Compile-fail tests MUST match expected `.stderr` output exactly (error messages, spans, and formatting)
- MUST uncomment all test entries in `tests/progress.rs` upon completion
- MUST run `cargo test` in each project directory to verify full compliance

**Test progression:**
- Each test file contains implementation hints in comments — these MUST be followed
- Tests are ordered by difficulty and dependency — implement in numerical order
- When a test fails, fix the implementation before proceeding

**Rationale:** The workshop's test suite was designed to validate incremental correctness. Passing all tests proves production-grade behavior.

### V. Educational Clarity

Code MUST serve as a clear, idiomatic reference for learning procedural macros.

**Non-negotiables:**
- MUST follow Rust API Guidelines for naming and documentation
- MUST include doc comments on public macro entry points explaining usage
- MUST structure code logically: parsing → validation → code generation
- MUST use descriptive variable names (not `t`, `x`, `f` — use `field_name`, `field_type`, etc.)
- Complex transformations MUST include inline comments explaining the "why"
- Error paths MUST be as clear as success paths

**Code organization (per macro crate):**
```
src/
├── lib.rs           # Macro entry point with #[proc_macro_derive] or #[proc_macro_attribute]
├── parse.rs         # Input parsing and validation (optional, for complex inputs)
├── expand.rs        # Code generation logic (optional, for complex generation)
└── error.rs         # Error construction helpers (optional)
```

**Rationale:** This repository exists to teach. Code that is fast but unreadable defeats the purpose. Clarity enables learning.

## Technology Stack

**Language Version:** Rust stable (latest stable edition 2021)

**Macro Type Coverage:**
| Project | Macro Type | Key Learning | Recommended Helper Crates |
|---------|-----------|--------------|---------------------------|
| `builder` | Derive macro | Syntax traversal, code generation, helper attributes | darling (for `#[builder(...)]`), heck |
| `debug` | Derive macro | Generic bounds, lifetime handling, trait bounds inference | darling (for `#[debug = "..."]`) |
| `seq` | Function-like macro | Custom syntax parsing, token manipulation | (manual parsing preferred) |
| `sorted` | Attribute macro | Compile-time validation, visitor pattern, diagnostics | proc-macro-error2 (for multiple errors) |
| `bitfield` | Attribute macro | Multi-macro interaction, const evaluation, trait tricks | darling, proc-macro-error2 |

**Core Dependencies (all projects):**
```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing"] }  # Add "full" only if needed
quote = "1"
proc-macro2 = "1"
```

**Optional Dependencies (add per-project as needed):**
```toml
# For complex attribute parsing (builder, debug, bitfield)
darling = "0.20"

# For ergonomic multi-error handling (sorted, bitfield)
proc-macro-error2 = "2"

# For case conversion (builder - field names to method names)
heck = "0.5"
```

**Testing Framework:**
- `trybuild` — Compile-time testing harness (already configured)
- Test categories: compile-pass (success) and compile-fail (expected errors with .stderr)

## Quality Standards

**Before considering a project complete:**
- [ ] All tests in `tests/progress.rs` are uncommented and passing
- [ ] `cargo test` succeeds with no warnings in the project directory
- [ ] `cargo clippy` produces no warnings (run with `--all-targets`)
- [ ] `cargo doc` generates documentation without warnings
- [ ] `cargo expand` (on main.rs examples) produces readable, correct code
- [ ] Error messages point to the correct source span (not the macro invocation site generically)

**Performance validation:**
- Macro expansion SHOULD complete in < 100ms for typical inputs (not measurable via trybuild, but keep implementations lean)
- Generated code SHOULD not significantly impact downstream compile times

**Edge case checklist (per macro):**
- [ ] Empty structs/enums
- [ ] Unit structs
- [ ] Tuple structs (where applicable)
- [ ] Generic parameters: lifetimes, types, const generics
- [ ] Where clauses with complex bounds
- [ ] Multiple generic parameters
- [ ] Visibility modifiers (pub, pub(crate), etc.)
- [ ] Doc comments and attributes on fields/variants
- [ ] Reserved keywords that require raw identifiers (r#type, etc.)

## Governance

This constitution governs all AI-generated implementations in the proc-macro-workshop repository.

**Amendment procedure:**
1. Identify the principle requiring change
2. Document the rationale for amendment
3. Update the constitution with the change
4. Increment version according to semantic versioning:
   - MAJOR: Principle removed or fundamentally redefined
   - MINOR: New principle added or existing principle significantly expanded
   - PATCH: Clarifications, typo fixes, non-semantic refinements
5. Re-validate all existing implementations against new principles

**Compliance verification:**
- Every implementation PR MUST pass all tests before merge
- Code review MUST verify adherence to all five Core Principles
- When in doubt, Principle I (Compile-Time Efficiency) and Principle II (Production-Grade Completeness) take precedence

**Conflict resolution:**
- If efficiency conflicts with clarity → prefer clarity with a comment explaining the efficiency trade-off
- If completeness conflicts with scope → implement completeness; macros that partially work are worse than not having them

**Runtime guidance:** Use `cargo expand`, `eprintln!` debugging (during development), and `trybuild` output to verify macro behavior.

**Version**: 1.1.0 | **Ratified**: 2025-12-25 | **Last Amended**: 2025-12-25
