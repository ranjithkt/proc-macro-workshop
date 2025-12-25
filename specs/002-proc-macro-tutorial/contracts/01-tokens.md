# Contract: Chapter 01 - Understanding Tokens

## Purpose
Establish the foundational concept that proc-macros operate on tokens, not text or AST.

## Content Requirements

### What You'll Learn
- What TokenStream and TokenTree are
- The 4 TokenTree variants: Ident, Punct, Group, Literal
- Why proc-macro2 exists
- How to debug TokenStreams with eprintln!

### Sections

1. **Introduction: Macros See Tokens, Not Text** (~200 words)
   - Analogy: macros as token transformers
   - Why this matters for understanding the ecosystem

2. **The proc-macro Crate** (~400 words)
   - TokenStream: the input and output type
   - TokenTree enum and its 4 variants
   - Code example: printing tokens with eprintln!
   ```rust
   #[proc_macro]
   pub fn debug_tokens(input: TokenStream) -> TokenStream {
       eprintln!("Tokens: {:#?}", input);
       input
   }
   ```
   - Show output for `struct Foo { x: i32 }`

3. **Diagram: TokenStream Structure** (FR-010)
   - Mermaid flowchart showing code → tokens
   ```mermaid
   graph TD
       A["struct Foo { x: i32 }"] --> B[TokenStream]
       B --> C1[Ident: struct]
       B --> C2[Ident: Foo]
       B --> C3[Group: braces]
       C3 --> D1[Ident: x]
       C3 --> D2[Punct: :]
       C3 --> D3[Ident: i32]
   ```

4. **Why proc-macro2 Exists** (~300 words)
   - Limitation: proc-macro only works in proc-macro crates
   - Solution: proc-macro2 provides the same types anywhere
   - Conversion: `.into()` between the two
   - This is why syn and quote use proc-macro2

5. **Code Example: Complete Token Inspector** (~200 words)
   - Full example that prints each token type
   - Expected output shown

6. **Key Takeaways**
   - Macros receive TokenStream, return TokenStream
   - TokenTree has 4 variants
   - proc-macro2 enables testing and library code
   - eprintln! is your friend for debugging

### Diagrams Required
- [x] TokenStream structure (FR-010)

## Estimated Time
15 minutes

## Dependencies
None (first chapter)

## Acceptance Criteria
- Reader can explain TokenTree variants
- Reader can write eprintln! debugging
- Reader understands proc-macro vs proc-macro2

