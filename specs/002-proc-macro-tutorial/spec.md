# Feature Specification: Proc-Macro Tutorial Documentation

**Feature Branch**: `002-proc-macro-tutorial`  
**Created**: 2025-12-25  
**Status**: Draft  
**Input**: Create educational documentation explaining proc-macro concepts from all crates used (proc-macro, proc-macro2, syn, quote, darling) with textbook-style flow, code examples, and eprintln debugging demonstrations

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

### User Story 5 - Understanding the Complete Pipeline (Priority: P2)

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
- **FR-005**: Documentation MUST cover darling's traits: FromDeriveInput, FromField, FromMeta, and common attributes
- **FR-006**: Documentation MUST include runnable code examples that demonstrate eprintln debugging of TokenStreams and parsed structures
- **FR-007**: Documentation MUST be organized as multiple linked markdown files with a clear progression from foundational to advanced concepts
- **FR-008**: Each chapter MUST include a "What You'll Learn" summary and a "Key Takeaways" conclusion
- **FR-009**: Documentation MUST include visual diagrams (ASCII or Mermaid) showing type relationships and data flow
- **FR-010**: Documentation MUST maintain an entertaining, conversational tone while being technically accurate
- **FR-011**: Code examples MUST show both the Rust code being parsed AND the output of eprintln debugging
- **FR-012**: Documentation MUST explain how each crate solves problems that the previous crate left unsolved (building narrative)

### Key Entities

- **Chapter**: A standalone markdown file covering one major concept, with prerequisites listed
- **Code Example**: A compilable snippet with input code, macro code, and expected output (both success and debug prints)
- **Concept**: A named idea (e.g., "TokenTree", "DeriveInput") with definition, examples, and connections to other concepts
- **Crate**: One of the 5 crates being documented, with its purpose, key types, and role in the ecosystem

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reader with basic Rust knowledge can follow the tutorial from start to finish in under 2 hours
- **SC-002**: Each chapter can be read independently in under 20 minutes
- **SC-003**: 100% of code examples compile and produce the documented output
- **SC-004**: A reader completing the tutorial can write a basic derive macro from scratch without referring back to documentation
- **SC-005**: The documentation covers all types we actually used in the proc-macro-workshop implementations

## Assumptions

- Readers have basic Rust syntax knowledge (structs, enums, traits, generics)
- Readers do not need prior proc-macro experience
- The documentation targets Rust 2021 edition
- Code examples use the same crate versions as the proc-macro-workshop (syn 2.x, quote 1.x, darling 0.20.x)
- Readers have access to cargo and can run code examples locally
