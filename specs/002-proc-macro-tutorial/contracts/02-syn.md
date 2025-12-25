# Contract: Chapter 02 - Parsing with syn

## Purpose
Show how syn transforms raw tokens into a typed, structured AST.

## Content Requirements

### What You'll Learn
- How syn parses TokenStream into structured types
- Key types: DeriveInput, Data, Fields, Attribute
- The parse_macro_input! macro
- Navigating the syn type hierarchy

### Sections

1. **The Problem: Tokens Are Too Low-Level** (~150 words)
   - Manually matching TokenTree variants is tedious
   - We want structured access to fields, types, attributes
   - syn solves this

2. **DeriveInput: Your Entry Point** (~400 words)
   - parse_macro_input! usage
   - DeriveInput structure: ident, generics, data
   - Code example:
   ```rust
   let input = parse_macro_input!(tokens as DeriveInput);
   eprintln!("Struct name: {}", input.ident);
   ```

3. **The Data Enum** (~300 words)
   - Three variants: Struct, Enum, Union
   - Pattern matching to handle each
   - Focus on DataStruct for derive macros

4. **Diagram: syn Type Hierarchy** (FR-011)
   ```mermaid
   classDiagram
       DeriveInput --> Data
       DeriveInput --> Generics
       Data --> DataStruct
       Data --> DataEnum
       DataStruct --> Fields
       Fields --> Field
       Field --> Ident
       Field --> Type
       Field --> Attribute
   ```

5. **Working with Fields** (~300 words)
   - Fields enum: Named, Unnamed, Unit
   - Iterating over fields
   - Accessing field ident, ty, attrs
   - Code example: printing all field names and types

6. **Parsing Attributes** (~250 words)
   - Attribute structure
   - Meta enum: Path, List, NameValue
   - Example: parsing `#[debug = "..."]`

7. **Feature Flags** (~150 words)
   - `derive` for DeriveInput
   - `parsing` for Parse trait
   - `full` for expressions (avoid unless needed)
   - `extra-traits` for Debug (dev only!)

8. **Key Takeaways**
   - syn turns tokens into typed structures
   - DeriveInput → Data → Fields → Field
   - Parse only what you need (feature flags)
   - eprintln!("{:#?}", input) for debugging

### Diagrams Required
- [x] syn type hierarchy (FR-011)

## Estimated Time
20 minutes

## Dependencies
Chapter 01 (tokens)

## Acceptance Criteria
- Reader can parse a struct with syn
- Reader can iterate over fields
- Reader understands the type hierarchy

