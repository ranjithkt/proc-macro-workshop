# Contract: Chapter 06 - The Complete Pipeline

## Purpose
Synthesize all concepts into a unified mental model with a complete macro walkthrough.

## Content Requirements

### What You'll Learn
- How all crates fit together
- The macro expansion pipeline
- Common patterns and idioms
- Reference architecture for derive macros

### Sections

1. **The Big Picture** (~200 words)
   - Macros as code transformers
   - Input → Parse → Transform → Generate → Output
   - Each crate handles one stage

2. **Diagram: The Macro Pipeline** (FR-012)
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
       subgraph Output
           F --> G[proc-macro<br/>TokenStream]
           G --> H[Generated Code]
       end
   ```

3. **Walkthrough: Builder Derive Macro** (~600 words)
   - Complete annotated implementation
   - Comments mapping to tutorial chapters
   ```rust
   // Chapter 01: Entry point receives TokenStream
   #[proc_macro_derive(Builder, attributes(builder))]
   pub fn derive(input: TokenStream) -> TokenStream {
       // Chapter 02: syn parses to structured types
       let input = parse_macro_input!(input as DeriveInput);
       
       // Chapter 04: darling extracts attributes
       match BuilderInput::from_derive_input(&input) {
           Ok(parsed) => {
               // Chapter 03 & 05: quote generates code
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
   - **Pattern 4**: Error accumulation

5. **Error Handling Patterns** (~300 words)
   - syn::Error for simple cases
   - quote_spanned! for location
   - darling's built-in error handling
   - Collecting multiple errors

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
   # Optional:
   darling = "0.20"  # Complex attributes
   heck = "0.5"      # Case conversion
   ```

8. **Key Takeaways**
   - Pipeline: TokenStream → syn → transform → quote → TokenStream
   - Each crate solves one problem well
   - Combine them for powerful, maintainable macros
   - Test with trybuild, debug with cargo expand

### Diagrams Required
- [x] Complete pipeline flowchart (FR-012)

## Estimated Time
20 minutes

## Dependencies
All previous chapters (synthesis)

## Acceptance Criteria
- Reader has unified mental model
- Reader can trace code through pipeline
- Reader has reference for own macros

