# Contract: Chapter 03 - Code Generation with quote

## Purpose
Teach how to generate Rust code using the quote crate's quasi-quoting macros.

## Content Requirements

### What You'll Learn
- The quote! macro for generating code
- Variable interpolation with #
- Repetition with #(...)* 
- Preserving spans with quote_spanned!
- Creating identifiers with format_ident!

### Sections

1. **The Problem: Building TokenStreams Manually** (~150 words)
   - TokenStream::from_iter is verbose
   - We want to write Rust-like syntax
   - quote provides quasi-quoting

2. **Basic quote! Usage** (~300 words)
   - Writing code that looks like Rust
   - Variables are interpolated with #name
   - Example: generating a function
   ```rust
   let name = &input.ident;
   quote! {
       impl #name {
           fn hello() { println!("Hello!"); }
       }
   }
   ```

3. **Variable Interpolation** (~300 words)
   - #var for single values
   - Types that implement ToTokens
   - Ident, Type, Literal all work
   - Example: interpolating field type

4. **Repetition Syntax** (~400 words)
   - #(#item)* for iteration
   - #(#item),* with separator
   - Nested repetition #(#(#inner)*)* 
   - Example: generating methods for each field
   ```rust
   let fields = &data.fields;
   quote! {
       #(
           pub fn #field_names(&self) -> &#field_types {
               &self.#field_names
           }
       )*
   }
   ```

5. **quote_spanned! for Better Errors** (~250 words)
   - Why spans matter for error messages
   - quote_spanned! preserves source locations
   - Example: error pointing to specific field
   ```rust
   quote_spanned! { field.span() =>
       compile_error!("field must be public");
   }
   ```

6. **format_ident! for Dynamic Names** (~200 words)
   - Creating identifiers from strings
   - Combining with other identifiers
   - Example: `format_ident!("get_{}", field_name)`

7. **Putting It Together: Debug Derive** (~300 words)
   - Complete example generating Debug impl
   - Shows parsing → transformation → generation
   - Full code with eprintln output

8. **Key Takeaways**
   - quote! generates TokenStream from Rust-like syntax
   - # interpolates variables
   - #(...)* for repetition
   - quote_spanned! for accurate error locations

### Diagrams Required
None (code-focused chapter)

## Estimated Time
20 minutes

## Dependencies
Chapter 02 (syn)

## Acceptance Criteria
- Reader can generate impl blocks
- Reader can iterate over fields with #(...)*
- Reader understands span preservation

