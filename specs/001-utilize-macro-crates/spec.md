# Feature Specification: Fully Utilize Darling, Proc-Macro-Error2, and Heck Crates

**Feature Branch**: `001-utilize-macro-crates`  
**Created**: 2025-12-25  
**Status**: Draft  
**Input**: User description: "I do not see full utilization of darling, proc-macro-error2 and heck crates in each of the projects in this repo. We need to fully utilize all their capabilities (to the point where they simplify the code written for proc macros). I want you to analyze each project's lib, go through line by line and figure out to simplify code using these 3 additional crates."

## Clarifications

### Session 2025-12-25

- Q: Should all three crates be added to every project, or only where they provide measurable simplification? → A: Add only dependencies that simplify existing code (skip if no clear use case)
- Q: Should error messages remain identical after refactoring, or can wording change? → A: Semantic equivalence required (same span + same meaning, wording may differ)
- Q: How should the 30% code reduction be measured? → A: Logical statements in parsing-related function bodies (excluding whitespace/comments)
- Q: What files are in scope for refactoring? → A: Focus on lib.rs files; tests may be updated if error crate usage extends beyond natural patterns

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Simplified Attribute Parsing with Darling (Priority: P1)

As a proc macro developer, I want all attribute parsing to leverage darling's full capabilities so that boilerplate code for manual attribute extraction is eliminated and error messages are automatically generated with proper spans.

**Why this priority**: Attribute parsing is the foundation of most proc macros. Simplifying this reduces the largest portion of repetitive code and improves error quality across all projects.

**Independent Test**: Can be tested by verifying all proc macros still pass their existing test suites while having reduced lines of code for attribute parsing.

**Acceptance Scenarios**:

1. **Given** a proc macro with manual attribute parsing logic, **When** darling's `FromDeriveInput`, `FromField`, `FromMeta`, or `FromVariant` traits are applied, **Then** the manual parsing code is replaced with declarative derive macros.
2. **Given** a proc macro that manually extracts nested attribute values, **When** darling's nested attribute support is used, **Then** the nested parsing is handled declaratively via `#[darling(attributes(...))]`.
3. **Given** a proc macro that manually validates attribute values, **When** darling's `#[darling(map = "...")]` or validation features are used, **Then** validation code is moved to the type definition.

---

### User Story 2 - Enhanced Error Handling with Proc-Macro-Error2 (Priority: P2)

As a proc macro developer, I want all error handling to use proc-macro-error2's capabilities so that error reporting is cleaner, errors can be accumulated rather than failing on the first one, and the code is more maintainable.

**Why this priority**: Better error handling improves the user experience when proc macros fail, and reduces the boilerplate of manually constructing `syn::Error` and converting to `TokenStream`.

**Independent Test**: Can be tested by verifying that proc macros produce the same or better error messages, and the error handling code is simplified.

**Acceptance Scenarios**:

1. **Given** a proc macro that uses `syn::Error::new(...).to_compile_error()`, **When** proc-macro-error2 is integrated, **Then** the macro uses `abort!` or `abort_call_site!` for cleaner error handling.
2. **Given** a proc macro that returns early on the first error, **When** proc-macro-error2's `emit_error!` is used, **Then** the macro can accumulate and report multiple errors at once.
3. **Given** a proc macro entry point, **When** the `#[proc_macro_error]` attribute is applied, **Then** error handling boilerplate is eliminated from the function body.

---

### User Story 3 - Case Conversion with Heck (Priority: P3)

As a proc macro developer, I want case conversion operations to use the heck crate so that identifier transformations (snake_case, camelCase, PascalCase, etc.) are standardized and the code is more readable.

**Why this priority**: Case conversion is a common operation in proc macros, and using a dedicated crate ensures consistency and reduces bugs in manual string manipulation.

**Independent Test**: Can be tested by verifying that generated identifiers maintain the same naming conventions while using heck's conversion methods.

**Acceptance Scenarios**:

1. **Given** a proc macro that manually formats identifier names, **When** heck's case conversion traits are applied, **Then** the string formatting code is replaced with trait method calls like `to_snake_case()` or `to_upper_camel_case()`.
2. **Given** a proc macro that generates getter/setter method names, **When** heck is used, **Then** the naming logic uses `to_snake_case()` for consistency.
3. **Given** a proc macro that needs to generate different case variants of the same name, **When** heck's traits are imported, **Then** all case conversions use the standardized heck methods.

---

### Edge Cases

- What happens when darling parsing fails for a required attribute? (Darling should provide automatic error messages with proper spans)
- What happens when a user provides an invalid attribute value? (proc-macro-error2 should enable clear, accumulated error messages)
- What happens with non-ASCII identifiers during case conversion? (heck should handle Unicode properly)
- What happens when multiple errors occur in the same macro invocation? (proc-macro-error2 should report all errors, not just the first)

## Requirements *(mandatory)*

### Functional Requirements

#### Builder Project

- **FR-001**: The builder project MUST use darling's `FromField` with `#[darling(map = "...")]` for type extraction helpers instead of custom `get_inner_type` functions where appropriate
- **FR-002**: The builder project MUST add proc-macro-error2 for cleaner error handling in the derive macro entry point
- **FR-003**: The builder project MUST use heck for any identifier case conversions (e.g., builder struct naming)

#### Debug Project

- **FR-004**: The debug project MUST use darling's `SpannedValue<T>` for attribute values that need span information for error reporting
- **FR-005**: The debug project MUST use darling's `FromMeta` for parsing the `#[debug(bound = "...")]` nested attribute instead of manual parsing
- **FR-006**: The debug project MUST add proc-macro-error2 for improved error accumulation during bounds analysis
- **FR-007**: The debug project MUST add heck as a dependency (currently missing) for any case-related operations

#### Seq Project

- **FR-008**: The seq project SHOULD add darling only if it simplifies the `SeqInput` parsing (function-like macros may not benefit from darling's derive-focused design)
- **FR-009**: The seq project MUST add proc-macro-error2 for cleaner error handling when parsing fails
- **FR-010**: The seq project SHOULD add heck only if case conversion is actively used in identifier generation

#### Sorted Project

- **FR-011**: The sorted project SHOULD add darling only if it simplifies attribute parsing (currently minimal attribute usage)
- **FR-012**: The sorted project MUST add proc-macro-error2 with `#[proc_macro_error]` attribute for cleaner error handling
- **FR-013**: The sorted project SHOULD add heck only if case conversion is actively used
- **FR-014**: The sorted project MUST replace manual `syn::Error::new().to_compile_error()` patterns with proc-macro-error2's `abort!` macro

#### Bitfield-Impl Project

- **FR-015**: The bitfield-impl project MUST fully utilize darling's `FromDeriveInput` and `FromField` for parsing struct fields and attributes
- **FR-016**: The bitfield-impl project MUST use darling for parsing the `#[bits = N]` attribute instead of manual `get_bits_attribute` function
- **FR-017**: The bitfield-impl project MUST add proc-macro-error2 for error handling in both `bitfield` and `BitfieldSpecifier` macros
- **FR-018**: The bitfield-impl project MUST add heck as a dependency for getter/setter method name generation (e.g., `get_field_name`, `set_field_name`)

### Key Entities

- **Proc Macro Projects**: builder, debug, seq, sorted, bitfield-impl - each containing procedural macro implementations
- **Darling Integration Points**: Struct/field attribute parsing, nested attributes, type validation
- **Error Handling Points**: Entry functions, parsing failures, validation errors
- **Case Conversion Points**: Generated method names, struct names, identifier transformations

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each proc macro project includes darling, proc-macro-error2, and/or heck only where they provide measurable code simplification
- **SC-002**: Total logical statements in manual attribute parsing code reduced by at least 30% across all projects (measured by parsing-related function bodies, excluding whitespace/comments)
- **SC-003**: All proc macro entry points use `#[proc_macro_error]` attribute from proc-macro-error2
- **SC-004**: All manual `syn::Error::new(...).to_compile_error()` patterns replaced with proc-macro-error2 macros
- **SC-005**: All case conversion operations use heck's trait methods instead of manual string formatting
- **SC-006**: All existing tests continue to pass after the refactoring
- **SC-007**: Error messages maintain semantic equivalence (same span location, same diagnostic meaning; exact wording may differ)
- **SC-008**: No increase in compile time of more than 10% for the proc macro crates

## Scope Boundaries

**In Scope:**
- `lib.rs` files in each proc macro crate (builder, debug, seq, sorted, bitfield-impl)
- Test files only if error crate usage extends beyond natural patterns

**Out of Scope:**
- Non-proc-macro library files (e.g., `bitfield/src/lib.rs` which only re-exports)
- Documentation files
- Build configuration beyond dependency additions

## Assumptions

- The proc-macro-error2 crate is a maintained fork/successor of proc-macro-error that works with current Rust editions
- The existing test suites are comprehensive enough to catch any behavioral regressions
- Performance impact of additional dependencies is acceptable given the code simplification benefits
- The darling crate's error messages are sufficient quality to replace custom error handling in most cases
