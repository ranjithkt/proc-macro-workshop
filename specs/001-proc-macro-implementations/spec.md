# Feature Specification: Proc-Macro Workshop Implementations

**Feature Branch**: `001-proc-macro-implementations`  
**Created**: 2025-12-25  
**Status**: Draft  
**Input**: User description: "Implement all 5 proc-macro workshop projects: builder, debug, seq, sorted, and bitfield with production-grade code that passes all tests"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Builder Macro Implementation (Priority: P1)

As a Rust developer, I want to use `#[derive(Builder)]` on a struct so that I can construct instances using the builder pattern without writing boilerplate code manually.

**Why this priority**: This is the recommended first project for learning proc-macros. It covers fundamental skills (syntax tree traversal, code generation, helper attributes) that are prerequisites for all other projects.

**Independent Test**: Can be fully tested by running `cargo test` in the `builder/` directory after uncommenting all tests in `tests/progress.rs`. Delivers a complete, working derive macro.

**Acceptance Scenarios**:

1. **Given** a struct with named fields, **When** `#[derive(Builder)]` is applied, **Then** a `builder()` method and a `CommandBuilder` struct with setter methods are generated
2. **Given** a struct with `Option<T>` fields, **When** the builder is used, **Then** optional fields can be omitted and default to `None`
3. **Given** a field with `#[builder(each = "name")]` attribute on a `Vec<T>` field, **When** the builder is used, **Then** a method accepting single items is generated alongside (or replacing) the batch method
4. **Given** an unrecognized attribute like `#[builder(unknown)]`, **When** the macro is compiled, **Then** a clear compile-time error with correct span pointing to the invalid attribute is produced
5. **Given** a struct using redefined prelude types (e.g., local `Option`), **When** the builder is used, **Then** the generated code uses fully-qualified paths and works correctly

---

### User Story 2 - CustomDebug Macro Implementation (Priority: P2)

As a Rust developer, I want to use `#[derive(CustomDebug)]` on a struct to implement `std::fmt::Debug` with customizable field formatting via `#[debug = "format"]` attributes.

**Why this priority**: Builds on User Story 1 by introducing generic type parameter handling and trait bound inference—critical skills for production-grade macros.

**Independent Test**: Can be fully tested by running `cargo test` in the `debug/` directory. Demonstrates proper handling of generics, associated types, and escape-hatch bounds.

**Acceptance Scenarios**:

1. **Given** a struct with fields, **When** `#[derive(CustomDebug)]` is applied, **Then** `Debug` is implemented with standard field-by-field output
2. **Given** a field with `#[debug = "0b{:08b}"]`, **When** the struct is printed, **Then** that field uses the specified format string
3. **Given** a generic struct `Wrapper<T>`, **When** derived, **Then** correct trait bounds are inferred (T: Debug where T is used)
4. **Given** `PhantomData<T>` fields, **When** derived, **Then** no unnecessary `T: Debug` bound is added for phantom data
5. **Given** `#[debug(bound = "T::Value: Debug")]` on a struct with associated types, **When** derived, **Then** the custom bound replaces inference and the macro compiles

---

### User Story 3 - Seq Macro Implementation (Priority: P3)

As a Rust developer, I want to use `seq!(N in 0..512 { ... })` to generate repetitive code with sequential indices, enabling patterns like enum variant generation without manual repetition.

**Why this priority**: Introduces custom syntax parsing (not standard Rust syntax) and token manipulation—a different skill set from derive macros.

**Independent Test**: Can be fully tested by running `cargo test` in the `seq/` directory. Demonstrates ability to generate hundreds of sequential items.

**Acceptance Scenarios**:

1. **Given** `seq!(N in 0..8 { ... })`, **When** the macro is invoked, **Then** it parses the header correctly (identifier, range)
2. **Given** code using `~N` inside the body, **When** expanded, **Then** identifiers are pasted with the sequence number (e.g., `Cpu~N` becomes `Cpu0`, `Cpu1`, ...)
3. **Given** `#(...)*` repeat sections, **When** expanded, **Then** only the marked section is repeated while outer code appears once
4. **Given** `0..=16` (inclusive range), **When** expanded, **Then** the range includes both endpoints
5. **Given** errors in macro usage, **When** compiled, **Then** error spans point to the correct token locations

---

### User Story 4 - Sorted Macro Implementation (Priority: P4)

As a Rust developer, I want to use `#[sorted]` on enums and match expressions to enforce alphabetical ordering of variants/arms at compile time, catching ordering mistakes early.

**Why this priority**: Introduces compile-time validation and diagnostics as the primary output (rather than code generation), plus the visitor pattern for syntax tree traversal.

**Independent Test**: Can be fully tested by running `cargo test` in the `sorted/` directory. Demonstrates compile-time checking with precise error messages.

**Acceptance Scenarios**:

1. **Given** an enum with sorted variants, **When** `#[sorted]` is applied, **Then** the enum compiles successfully
2. **Given** an enum with unsorted variants, **When** compiled, **Then** a compile error identifies which variant is out of order and where it should go
3. **Given** `#[sorted]` on a non-enum item, **When** compiled, **Then** an error indicates that sorted only works on enums
4. **Given** `#[sorted::check]` on a function containing `#[sorted]` match expressions, **When** compiled, **Then** match arm order is validated and the inner `#[sorted]` attribute is stripped
5. **Given** match patterns using paths (e.g., `Error::Io`), **When** checked, **Then** the macro correctly extracts and compares variant names

---

### User Story 5 - Bitfield Macro Implementation (Priority: P5)

As a systems programmer, I want to use `#[bitfield]` to define packed binary structures with getters/setters for bit ranges, enabling low-level hardware register manipulation without manual bit twiddling.

**Why this priority**: Most complex project requiring multiple macro types (attribute macro + derive macro), trait implementations, compile-time arithmetic, and advanced const evaluation. Recommended only after mastering other projects.

**Independent Test**: Can be fully tested by running `cargo test` in the `bitfield/` directory. Demonstrates multi-macro coordination and compile-time validation.

**Acceptance Scenarios**:

1. **Given** types `B1` through `B64`, **When** used as field types, **Then** they implement `Specifier` trait with correct `BITS` values
2. **Given** a bitfield struct with total bits not multiple of 8, **When** compiled, **Then** a compile error is produced
3. **Given** a bitfield struct, **When** the macro expands, **Then** a `new()` method and `get_`/`set_` methods are generated with correct signatures
4. **Given** `#[derive(BitfieldSpecifier)]` on an enum with power-of-two variants, **When** used in a bitfield, **Then** getters/setters use the enum type directly
5. **Given** `#[bits = N]` attribute on a field, **When** compiled, **Then** the specified width is validated against the actual specifier width

---

### Edge Cases

- Empty structs (zero fields)
- Unit structs and tuple structs (where applicable)
- Structs/enums with generics: lifetime parameters, type parameters, const generics
- Complex where clauses with multiple bounds
- Visibility modifiers: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`
- Reserved Rust keywords requiring raw identifiers (`r#type`, `r#match`)
- Attributes that should be preserved vs. consumed
- `#[cfg(...)]` attributes on fields/variants
- Unicode identifiers in field/variant names
- Very long identifiers approaching compiler limits
- Nested types (e.g., `Option<Vec<Option<T>>>`)
- Self-referential types through `PhantomData`

## Requirements *(mandatory)*

### Functional Requirements

#### All Projects

- **FR-001**: All macros MUST pass 100% of their project's test suite in `tests/progress.rs`
- **FR-002**: Compile-fail tests MUST produce error messages matching the `.stderr` files exactly (spans, wording)
- **FR-003**: Generated code MUST use fully-qualified paths to avoid conflicts with user-defined types (e.g., `::std::option::Option`)
- **FR-004**: Error messages MUST point to the precise source location (field, attribute, variant) causing the problem
- **FR-005**: Macros MUST be hygienic—generated identifiers must not collide with user code

#### Builder (`derive_builder`)

- **FR-010**: MUST generate a `builder()` associated function returning `<Name>Builder`
- **FR-011**: MUST generate setter methods using `&mut self -> &mut Self` pattern for chaining
- **FR-012**: MUST handle `Option<T>` fields as optional (no setter required)
- **FR-013**: MUST support `#[builder(each = "name")]` for `Vec<T>` fields to enable one-at-a-time insertion
- **FR-014**: MUST produce compile errors for unrecognized `#[builder(...)]` attributes

#### CustomDebug (`derive_debug`)

- **FR-020**: MUST implement `std::fmt::Debug` trait correctly
- **FR-021**: MUST support `#[debug = "format_string"]` for custom field formatting
- **FR-022**: MUST infer correct trait bounds for generic type parameters
- **FR-023**: MUST NOT add `T: Debug` bound for `PhantomData<T>` fields
- **FR-024**: MUST support `#[debug(bound = "...")]` to override inferred bounds

#### Seq (`seq!`)

- **FR-030**: MUST parse `N in START..END` header syntax
- **FR-031**: MUST support inclusive ranges `START..=END`
- **FR-032**: MUST support identifier pasting with `~` (e.g., `Cpu~N`)
- **FR-033**: MUST support `#(...)*` sections for selective repetition
- **FR-034**: MUST handle interaction with `macro_rules!` macros correctly

#### Sorted (`sorted`)

- **FR-040**: MUST validate enum variants are in alphabetical order
- **FR-041**: MUST produce clear error identifying the out-of-order variant
- **FR-042**: MUST reject non-enum items with appropriate error
- **FR-043**: MUST support `#[sorted::check]` for validating match expressions in functions
- **FR-044**: MUST strip inner `#[sorted]` attributes after validation

#### Bitfield (`bitfield`)

- **FR-050**: MUST provide `Specifier` trait with `BITS` associated constant
- **FR-051**: MUST provide types `B1` through `B64` implementing `Specifier`
- **FR-052**: MUST validate total bits is multiple of 8
- **FR-053**: MUST generate `new()`, `get_*()`, and `set_*()` methods
- **FR-054**: MUST support `#[derive(BitfieldSpecifier)]` for enums
- **FR-055**: MUST support `#[bits = N]` for compile-time width validation
- **FR-056**: MUST use smallest fitting integer type for accessors (`u8`, `u16`, `u32`, `u64`)

### Key Entities

- **DeriveInput**: The parsed syntax tree of a struct/enum being derived (common to all derive macros)
- **TokenStream**: The input/output representation for all procedural macros
- **Specifier Trait**: (bitfield) Marker trait with BITS constant for bit width specification
- **Builder Struct**: (builder) Generated companion struct holding optional field values

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All tests pass — `cargo test` succeeds in all 5 project directories with all progress.rs tests uncommented
- **SC-002**: Error message accuracy — All `.stderr` compile-fail tests match expected output exactly
- **SC-003**: Code quality — `cargo clippy --all-targets` produces zero warnings in all project directories
- **SC-004**: Documentation — `cargo doc` generates documentation without warnings
- **SC-005**: Real-world usage — Generated code from `cargo expand` is readable and matches expected patterns
- **SC-006**: Edge case coverage — All listed edge cases compile and behave correctly where applicable

## Assumptions

The following reasonable defaults and assumptions have been made:

1. **Rust Version**: Targeting latest stable Rust (edition 2021)
2. **Dependency Versions**: Using `syn 2.x`, `quote 1.x`, `proc-macro2 1.x` as specified in constitution
3. **Test Framework**: Using existing `trybuild` test harness already configured in each project
4. **Error Span Strategy**: Using `syn::Error` with proper spans for all compile-time errors
5. **Code Style**: Following Rust API Guidelines and existing workshop conventions
6. **Helper Crate Usage**: Using `darling` for complex attribute parsing where it simplifies code
7. **Project Order**: Implementing in the recommended order (builder → debug → seq → sorted → bitfield)
