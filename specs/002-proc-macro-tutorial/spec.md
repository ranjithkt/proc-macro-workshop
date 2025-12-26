# Feature Specification: Proc-Macro Tutorial Documentation

**Feature Branch**: `002-proc-macro-tutorial`  
**Created**: 2025-12-25  
**Updated**: 2025-12-25  
**Status**: Draft  
**Input**: Create educational documentation explaining proc-macro concepts from all crates used (proc-macro, proc-macro2, syn, quote, darling, proc-macro-error2, heck) with textbook-style flow, code examples, and eprintln debugging demonstrations

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Understanding the Token Foundation (Priority: P1)

A developer new to Rust procedural macros wants to understand what TokenStreams are and how the compiler sees their code. They need to grasp the foundational concept that macros operate on tokens, not text, before they can understand the higher-level abstractions.

**Why this priority**: This is the conceptual foundation. Without understanding TokenStreams and the proc-macro standard library, all other crates make no sense. This must come first.

**Independent Test**: Can be fully tested by a reader completing the chapter and successfully predicting what tokens `struct Foo { x: i32 }` produces. Delivers understanding of the "raw material" that all proc-macros work with.

**Acceptance Scenarios**:

1. **Given** a developer reading the TokenStream chapter, **When** they reach the end, **Then** they can explain what TokenTree, Ident, Group, Punct, and Literal are with examples
2. **Given** a code example with eprintln debugging, **When** the reader traces through the output, **Then** they understand how Rust code becomes a stream of tokens
3. **Given** the proc-macro vs proc-macro2 distinction, **When** explained, **Then** the reader understands why proc-macro2 exists and when to use each

---

### User Story 2 - Parsing with Syn (Priority: P1)

A developer understands tokens but finds them tedious to work with directly. They want to learn how the syn crate transforms raw tokens into a structured, typed Abstract Syntax Tree (AST) that's easier to inspect and manipulate.

**Why this priority**: Syn is the most critical tool in the proc-macro ecosystem. Every serious proc-macro uses it. It builds directly on TokenStream knowledge.

**Independent Test**: Can be fully tested by the reader writing a simple derive macro that parses a struct and prints its field names. Delivers practical parsing capability.

**Acceptance Scenarios**:

1. **Given** a reader who completed the TokenStream chapter, **When** they read about DeriveInput and Data enum, **Then** they understand how tokens become structured AST nodes
2. **Given** code examples showing struct parsing, **When** traced with eprintln, **Then** the reader sees the mapping from tokens to syn types
3. **Given** the different syn types (ItemStruct, ItemEnum, Fields, etc.), **When** explained with visuals, **Then** the reader can navigate the syn type hierarchy

---

### User Story 3 - Code Generation with Quote (Priority: P1)

A developer can parse Rust code with syn but needs to generate new Rust code as output. They want to learn how the quote crate provides an ergonomic way to construct TokenStreams using familiar Rust syntax.

**Why this priority**: Quote is the output side of the proc-macro equation. You can't write useful macros without generating code. This completes the core parse→transform→generate pipeline.

**Independent Test**: Can be fully tested by the reader generating a Debug impl for a parsed struct. Delivers practical code generation capability.

**Acceptance Scenarios**:

1. **Given** a reader who understands syn parsing, **When** they learn the quote! macro syntax, **Then** they can generate simple function or impl blocks
2. **Given** the #variable interpolation and #(#iter)* repetition syntax, **When** demonstrated, **Then** the reader can iterate over parsed fields to generate code
3. **Given** quote_spanned! examples, **When** error scenarios are shown, **Then** the reader understands how to preserve source locations for better errors

---

### User Story 4 - Ergonomic Attribute Parsing with Darling (Priority: P2)

A developer is manually parsing `#[attribute(key = "value")]` style attributes and finds the code repetitive and error-prone. They want to learn how darling declaratively derives attribute parsers from struct definitions.

**Why this priority**: Darling is an optional but highly valuable enhancement. It reduces boilerplate significantly but is not strictly necessary for basic macros.

**Independent Test**: Can be fully tested by the reader refactoring manual attribute parsing to use darling's FromDeriveInput and FromField traits. Delivers reduced code complexity.

**Acceptance Scenarios**:

1. **Given** a manual attribute parsing example (before), **When** refactored with darling (after), **Then** the reader sees the code reduction and improved error messages
2. **Given** the FromDeriveInput, FromField, and FromMeta traits, **When** explained, **Then** the reader knows which trait to use for struct-level vs field-level attributes
3. **Given** darling's built-in validations and defaults, **When** demonstrated, **Then** the reader can handle optional attributes elegantly

---

### User Story 5 - Utility Crates: heck for Case Conversion (Priority: P3)

A developer needs to transform identifier names (e.g., convert `field_name` to `setFieldName` or `FIELD_NAME`). They want to learn about helper crates like heck that make these transformations trivial.

**Why this priority**: heck is a small utility crate that solves a specific problem elegantly. Not essential but useful for real-world macros.

**Independent Test**: Can be fully tested by the reader using heck to convert identifiers in a builder pattern macro.

**Acceptance Scenarios**:

1. **Given** a need to convert snake_case to PascalCase, **When** heck is demonstrated, **Then** the reader can apply case conversions in their macros
2. **Given** multiple case conversion needs (camelCase, SCREAMING_SNAKE, etc.), **When** heck's API is shown, **Then** the reader knows the full range of available conversions

---

### User Story 6 - Ergonomic Error Handling with proc-macro-error2 (Priority: P2)

A developer is manually constructing `syn::Error` and calling `.to_compile_error()`, finding the pattern repetitive. They want to learn how proc-macro-error2 provides cleaner error handling with `abort!`, `emit_error!`, and the `#[proc_macro_error]` attribute.

**Why this priority**: Error handling is critical for production macros. This crate is used in all 5 workshop projects and is listed in the constitution as a recommended dependency.

**Independent Test**: Can be fully tested by the reader refactoring manual error handling to use proc-macro-error2's macros and attributes.

**Acceptance Scenarios**:

1. **Given** a macro using `syn::Error::new().to_compile_error()`, **When** refactored with proc-macro-error2, **Then** the code uses `abort!` or `abort_call_site!` for cleaner error handling
2. **Given** the `#[proc_macro_error]` attribute, **When** applied to entry point, **Then** error handling boilerplate is eliminated
3. **Given** multiple validation errors, **When** `emit_error!` is used, **Then** all errors are reported rather than failing on the first

---

### User Story 7 - Understanding the Complete Pipeline (Priority: P2)

A developer has learned the individual crates but wants to see how they fit together in a real-world macro. They need a visual and narrative that connects all the concepts into a coherent mental model.

**Why this priority**: This synthesis chapter reinforces learning and provides a reference architecture. Important for retention but requires prior chapters.

**Independent Test**: Can be fully tested by the reader tracing through a complete derive macro from input to output, identifying which crate handles each step.

**Acceptance Scenarios**:

1. **Given** a flowchart showing the macro pipeline, **When** studied, **Then** the reader can identify proc-macro → syn → transform → quote → proc-macro stages
2. **Given** a complete Builder derive macro, **When** annotated with comments, **Then** the reader can map each section to its corresponding tutorial chapter
3. **Given** common patterns and idioms, **When** catalogued, **Then** the reader has a reference for their own macros

---

### Edge Cases

- What happens when a reader skips chapters? Cross-references guide them back to prerequisites.
- How does documentation handle rustc version differences? Notes specify minimum Rust version and highlight edition-specific features.
- What if a reader wants to go deeper? "Learn More" sections link to official docs and source code.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Documentation MUST explain the proc-macro standard library crate including TokenStream, TokenTree, Ident, Group, Punct, and Literal types
- **FR-002**: Documentation MUST explain proc-macro2 and its relationship to the std proc-macro crate
- **FR-003**: Documentation MUST cover syn's core types: DeriveInput, Data, Fields, Attribute, Meta, and the parse module
- **FR-004**: Documentation MUST explain quote's macro syntax including variable interpolation (#var) and iteration (#(#iter)*)
- **FR-005**: Documentation MUST cover darling's traits: FromDeriveInput, FromField, FromMeta, and common attributes like forward_attrs
- **FR-006**: Documentation MUST cover proc-macro-error2's error handling: `abort!`, `emit_error!`, `abort_call_site!`, and the `#[proc_macro_error]` attribute
- **FR-007**: Documentation MUST cover heck crate for case conversions (snake_case, PascalCase, camelCase, etc.)
- **FR-008**: Documentation MUST include runnable code examples that demonstrate eprintln debugging of TokenStreams and parsed structures
- **FR-009**: Documentation MUST be organized as multiple linked markdown files with a clear progression from foundational to advanced concepts
- **FR-010**: Each chapter MUST include a "What You'll Learn" summary and a "Key Takeaways" conclusion

### Visual Diagram Requirements

- **FR-011**: Documentation MUST include a **TokenStream structure diagram** showing how code becomes tokens (ASCII art or Mermaid flowchart)
- **FR-012**: Documentation MUST include a **syn type hierarchy diagram** showing relationships between DeriveInput, Data, Fields, etc.
- **FR-013**: Documentation MUST include a **macro pipeline flowchart** showing: Input → proc-macro → syn parse → transform → quote → proc-macro → Output
- **FR-014**: Documentation MUST include **before/after code comparison diagrams** for darling refactoring examples
- **FR-015**: All diagrams MUST be renderable in standard markdown (Mermaid syntax preferred for compatibility)

### Tone and Style Requirements

- **FR-016**: Documentation MUST maintain an entertaining, conversational tone while being technically accurate
- **FR-017**: Code examples MUST show both the Rust code being parsed AND the output of eprintln debugging
- **FR-018**: Documentation MUST explain how each crate solves problems that the previous crate left unsolved (building narrative)
- **FR-019**: Documentation MUST include "Aha!" moments that highlight key insights

### Key Entities

- **Chapter**: A standalone markdown file covering one major concept, with prerequisites listed
- **Code Example**: A compilable snippet with input code, macro code, and expected output (both success and debug prints)
- **Diagram**: A visual representation (Mermaid or ASCII) embedded in markdown showing type relationships or data flow
- **Concept**: A named idea (e.g., "TokenTree", "DeriveInput") with definition, examples, and connections to other concepts
- **Crate**: One of the 7 crates being documented (proc-macro, proc-macro2, syn, quote, darling, proc-macro-error2, heck), with its purpose, key types, and role in the ecosystem

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reader with basic Rust knowledge can follow the tutorial from start to finish in under 2 hours of reading time (excluding hands-on exercises)
- **SC-002**: Each chapter can be read independently in under 20 minutes
- **SC-003**: 100% of code examples compile and produce the documented output
- **SC-004**: A reader completing the tutorial can write a basic derive macro from scratch without referring back to documentation
- **SC-005**: The documentation covers all types we actually used in the proc-macro-workshop implementations
- **SC-006**: Documentation contains at least 4 visual diagrams (TokenStream structure, syn hierarchy, pipeline flowchart, darling comparison)
- **SC-007**: Documentation covers all 7 crates used in the proc-macro-workshop implementations including proc-macro-error2

## Assumptions

- Readers have basic Rust syntax knowledge (structs, enums, traits, generics)
- Readers do not need prior proc-macro experience
- The documentation targets Rust 2021 edition
- Code examples use the same crate versions as the proc-macro-workshop:
  - syn 2.x
  - quote 1.x
  - proc-macro2 1.x
  - darling 0.20.x
  - proc-macro-error2 2.x
  - heck 0.5.x
- Readers have access to cargo and can run code examples locally
- Mermaid diagrams are viewable in GitHub, VS Code, and most modern markdown renderers
