# Contract: Chapter 06 - Error Handling with proc-macro-error2

## Purpose
Teach ergonomic error handling patterns that eliminate boilerplate and improve macro user experience.

## Content Requirements

### What You'll Learn
- Why manual `syn::Error` handling is tedious
- How `#[proc_macro_error]` simplifies entry points
- When to use `abort!` vs `emit_error!`
- Error accumulation patterns for better diagnostics

### Sections

1. **Introduction: Error Handling Matters** (~150 words)
   - Poor error messages frustrate users
   - Good error messages with correct spans save debugging time
   - The boilerplate problem with manual error handling

2. **The Problem: Manual Error Handling** (~200 words)
   ```rust
   // Every error requires this dance:
   return syn::Error::new(span, "message")
       .to_compile_error()
       .into();
   ```
   - Repetitive `.to_compile_error().into()` pattern
   - Early returns prevent collecting multiple errors
   - Verbose entry point error handling

3. **The Solution: proc-macro-error2** (~300 words)
   - Why proc-macro-error2 (not the original unmaintained crate)
   - The `#[proc_macro_error]` attribute
   - How it transforms the entry point
   ```rust
   #[proc_macro_derive(MyMacro)]
   #[proc_macro_error]
   pub fn derive(input: TokenStream) -> TokenStream {
       let input = parse_macro_input!(input as DeriveInput);
       // Errors handled automatically!
   }
   ```

4. **Core Macros** (~400 words)
   | Macro | Behavior | Use Case |
   |-------|----------|----------|
   | `abort!` | Stop immediately | Fatal errors |
   | `abort_call_site!` | Stop at macro call | Generic errors |
   | `emit_error!` | Continue processing | Accumulate errors |
   | `emit_call_site_error!` | Continue at call site | Generic warnings |

   - Code examples for each
   - Span preservation with abort!

5. **Before/After Comparison** (~200 words)
   - Side-by-side code comparison
   - Lines of code reduction
   - Improved readability

6. **Error Accumulation Pattern** (~200 words)
   ```rust
   for field in &fields {
       if !is_valid(field) {
           emit_error!(field.span(), "invalid field");
           // Continue checking other fields!
       }
   }
   // All errors reported at once
   ```
   - Why this improves user experience
   - When to use vs abort!

7. **Integration with darling** (~150 words)
   - darling's `write_errors()` pattern
   - When to use darling errors vs proc-macro-error2
   - Combining both approaches

8. **Key Takeaways**
   - `#[proc_macro_error]` goes on every entry point
   - `abort!` for fatal errors, `emit_error!` for accumulation
   - Use proc-macro-error2, not the unmaintained original
   - Better errors = happier macro users

### Diagrams Required
None (code-focused chapter)

## Estimated Time
12 minutes

## Dependencies
Chapter 03 (quote) - understands TokenStream output

## Acceptance Criteria
- Reader can refactor manual error handling to use abort!
- Reader understands when to accumulate vs abort
- Reader knows to use proc-macro-error2 (not the original)

