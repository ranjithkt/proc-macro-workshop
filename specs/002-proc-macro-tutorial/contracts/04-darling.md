# Contract: Chapter 04 - Ergonomic Attributes with darling

## Purpose
Show how darling simplifies attribute parsing through declarative derives.

## Content Requirements

### What You'll Learn
- Why manual attribute parsing is painful
- How darling derives parsers from structs
- FromDeriveInput, FromField, FromMeta traits
- Handling optional attributes and defaults

### Sections

1. **The Problem: Attribute Parsing Spaghetti** (~200 words)
   - Show deeply nested manual parsing code
   - 8+ levels of nesting for simple attributes
   - Error handling is manual and inconsistent

2. **Diagram: Before/After Darling** (FR-013)
   - Side-by-side comparison
   - Left: 30+ lines of manual parsing
   - Right: 10 lines with darling
   ```mermaid
   graph LR
       subgraph Before
           A[Match attr path] --> B[Match Meta::List]
           B --> C[Parse args]
           C --> D[Match NameValue]
           D --> E[Extract value]
       end
       subgraph After
           F[#derive FromField] --> G[Automatic parsing]
       end
   ```

3. **FromDeriveInput: Struct-Level Parsing** (~300 words)
   - Deriving FromDeriveInput
   - Accessing ident, generics, data
   - Parsing struct-level attributes
   ```rust
   #[derive(FromDeriveInput)]
   #[darling(attributes(my_macro))]
   struct MyInput {
       ident: Ident,
       data: Data<(), MyField>,
   }
   ```

4. **FromField: Field-Level Parsing** (~350 words)
   - Deriving FromField
   - Automatic ident and ty extraction
   - Custom attributes with #[darling(attributes(...))]
   ```rust
   #[derive(FromField)]
   #[darling(attributes(builder))]
   struct BuilderField {
       ident: Option<Ident>,
       ty: Type,
       #[darling(default)]
       each: Option<String>,
   }
   ```

5. **Darling Attributes Cheat Sheet** (~250 words)
   | Attribute | Purpose |
   |-----------|---------|
   | `attributes(name)` | Which helper attrs to parse |
   | `default` | Optional with Default value |
   | `rename = "x"` | Different attribute name |
   | `forward_attrs` | Preserve raw attributes |
   | `supports(...)` | Restrict to struct/enum |

6. **Error Handling Magic** (~200 words)
   - darling generates helpful error messages
   - "Did you mean?" suggestions for typos
   - Example: `eac` → "Did you mean `each`?"

7. **When NOT to Use Darling** (~150 words)
   - Simple single-value attributes
   - When you need custom error messages
   - Maximum control scenarios

8. **Key Takeaways**
   - darling is "serde for attributes"
   - FromDeriveInput for struct-level
   - FromField for field-level
   - Reduces code and improves errors

### Diagrams Required
- [x] Before/after comparison (FR-013)

## Estimated Time
15 minutes

## Dependencies
Chapter 03 (quote) - understands code generation context

## Acceptance Criteria
- Reader can refactor manual parsing to darling
- Reader knows which trait to use when
- Reader understands darling attributes

