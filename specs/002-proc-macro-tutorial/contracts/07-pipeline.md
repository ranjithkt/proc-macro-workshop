# Contract: Chapter 07 - The Complete Pipeline

## Purpose
Synthesize all concepts into a unified mental model with a complete macro walkthrough.

## Content Requirements

### What You'll Learn
- How all 7 crates fit together
- The macro expansion pipeline
- Common patterns and idioms
- Reference architecture for derive macros

### Sections

1. **The Big Picture** (~200 words)
   - Macros as code transformers
   - Input → Parse → Transform → Generate → Output
   - Each crate handles one stage

2. **Diagram: The Macro Pipeline** (FR-013)
   ```mermaid
   graph LR
       subgraph Input
           A[Rust Code] --> B[proc-macro<br/>TokenStream]
       end
       subgraph Parse
           B --> C[syn<br/>DeriveInput]
           C --> D[darling<br/>Typed Attrs]
       end
       subgraph Transform
           D --> E[Your Logic<br/>+ heck]
       end
       subgraph Generate
           E --> F[quote!<br/>TokenStream]
       end
       subgraph "Error Handling"
           F -.-> G[proc-macro-error2<br/>abort!/emit_error!]
       end
       subgraph Output
           F --> H[proc-macro<br/>TokenStream]
           H --> I[Generated Code]
       end
   ```

3. **Walkthrough: Builder Derive Macro** (~600 words)
   - Complete annotated implementation
   - Comments mapping to tutorial chapters
   ```rust
   // Chapter 01: Entry point receives TokenStream
   #[proc_macro_derive(Builder, attributes(builder))]
   #[proc_macro_error]  // Chapter 06: Error handling
   pub fn derive(input: TokenStream) -> TokenStream {
       // Chapter 02: syn parses to structured types
       let input = parse_macro_input!(input as DeriveInput);
       
       // Chapter 04: darling extracts attributes
       match BuilderInput::from_derive_input(&input) {
           Ok(parsed) => {
               // Chapter 03 & 05: quote + heck generates code
               generate_builder(parsed).into()
           }
           Err(e) => e.write_errors().into(),
       }
   }
   ```

4. **Common Patterns** (~400 words)
   - **Pattern 1**: Parse → Validate → Generate
   - **Pattern 2**: Struct + Field iteration
   - **Pattern 3**: Optional attribute with default
   - **Pattern 4**: Error accumulation with emit_error!

5. **Error Handling Patterns** (~300 words)
   - **Simple errors**: syn::Error for single error cases
   - **Span preservation**: quote_spanned! for location-aware errors
   - **Ergonomic errors**: proc-macro-error2's abort! and emit_error!
   - **Darling integration**: write_errors() for attribute parsing
   - **When to use each**: Decision tree for error handling approach

6. **Testing Your Macros** (~200 words)
   - cargo expand for debugging
   - trybuild for compile tests
   - eprintln! during development

7. **Reference: Macro Crate Checklist** (~150 words)
   ```toml
   [lib]
   proc-macro = true
   
   [dependencies]
   syn = { version = "2", features = ["derive", "parsing"] }
   quote = "1"
   proc-macro2 = "1"
   # Optional (but recommended):
   darling = "0.20"           # Complex attribute parsing
   proc-macro-error2 = "2"    # Ergonomic error handling
   heck = "0.5"               # Case conversion
   ```

8. **Key Takeaways**
   - Pipeline: TokenStream → syn → transform → quote → TokenStream
   - Each crate solves one problem well
   - Always use `#[proc_macro_error]` on entry points
   - Combine all 7 crates for powerful, maintainable macros
   - Test with trybuild, debug with cargo expand

### Diagrams Required
- [x] Complete pipeline flowchart (FR-013)

## Estimated Time
20 minutes

## Dependencies
All previous chapters (synthesis)

## Acceptance Criteria
- Reader has unified mental model
- Reader can trace code through pipeline
- Reader has reference for own macros

