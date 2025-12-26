# Chapter 6: Error Handling with proc-macro-error2 🚨

## What You'll Learn

- Why manual `syn::Error` handling is tedious
- How `#[proc_macro_error]` simplifies entry points
- When to use `abort!` vs `emit_error!`
- Error accumulation patterns for better diagnostics

---

## Introduction: Error Handling Matters

Good error messages are the difference between "I love this macro!" and "What is this cryptic garbage?!"

Consider the user experience:

**Bad macro error:**
```text
error[E0277]: the trait bound `Foo: Default` is not satisfied
  --> src/main.rs:1:10
   |
1  | #[derive(Builder)]
   |          ^^^^^^^ the trait `Default` is not implemented for `Foo`
```

**Good macro error:**
```text
error: field `config` requires a default value or must implement Default
  --> src/main.rs:5:5
   |
5  |     config: Foo,
   |     ^^^^^^
   |
   = help: add `#[builder(default)]` or implement Default for Foo
```

The second one tells you *what's wrong* and *how to fix it*. That's what we're aiming for.

---

## The Problem: Manual Error Handling

With just `syn`, error handling looks like this:

```rust
#[proc_macro_derive(MyMacro)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Every error requires this dance:
    if some_validation_fails(&input) {
        return syn::Error::new(some_span, "something went wrong")
            .to_compile_error()
            .into();
    }
    
    // And another...
    if another_check_fails(&input) {
        return syn::Error::new(another_span, "another thing went wrong")
            .to_compile_error()
            .into();
    }
    
    // Early returns everywhere!
    // Can't accumulate multiple errors!
    // .to_compile_error().into() repeated endlessly!
    
    generate_code(&input).into()
}
```

Three problems:
1. **Repetitive boilerplate**: `.to_compile_error().into()` everywhere
2. **One error at a time**: First error stops everything
3. **Verbose entry points**: Try/catch logic clutters the code

---

## The Solution: proc-macro-error2

`proc-macro-error2` (the maintained fork of `proc-macro-error`) solves all three:

```rust
use proc_macro_error2::{proc_macro_error, abort};

#[proc_macro_derive(MyMacro)]
#[proc_macro_error]  // <-- Magic happens here
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    if some_validation_fails(&input) {
        abort!(some_span, "something went wrong");  // Clean!
    }
    
    generate_code(&input).into()
}
```

The `#[proc_macro_error]` attribute:
- Catches panics from `abort!`
- Converts them to proper compile errors
- Handles multiple accumulated errors

**💡 Aha!** Why `proc-macro-error2`? The original `proc-macro-error` is unmaintained. The `2` version is actively maintained and compatible with current Rust.

---

## Core Macros

| Macro | Behavior | Use Case |
|-------|----------|----------|
| `abort!(span, msg)` | Stop immediately | Fatal errors |
| `abort_call_site!(msg)` | Stop at macro call | Generic errors |
| `emit_error!(span, msg)` | Continue processing | Accumulate errors |
| `emit_call_site_error!(msg)` | Continue at call site | Generic warnings |

### abort!: Stop Now

Use when there's no point continuing:

```rust
use proc_macro_error2::abort;

fn validate_struct(data: &Data) {
    let Data::Struct(s) = data else {
        abort!(data.span(), "only structs are supported");
    };
    // Continue with struct processing...
}
```

### emit_error!: Keep Going

Use when you want to report multiple errors:

```rust
use proc_macro_error2::emit_error;

fn validate_fields(fields: &[Field]) {
    for field in fields {
        if !is_valid(field) {
            emit_error!(field.span(), "invalid field configuration");
            // Continue checking other fields!
        }
    }
    // All errors reported at once when macro returns
}
```

---

## Before/After Comparison

### Before (Manual)

```rust
#[proc_macro_derive(MyMacro)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = match syn::parse::<DeriveInput>(input) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };
    
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "expected struct")
            .to_compile_error()
            .into();
    };
    
    for field in &data.fields {
        if !is_valid(field) {
            return syn::Error::new_spanned(field, "invalid field")
                .to_compile_error()
                .into();
        }
    }
    
    generate_code(&input).into()
}
```

### After (proc-macro-error2)

```rust
use proc_macro_error2::{proc_macro_error, abort, emit_error};

#[proc_macro_derive(MyMacro)]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let Data::Struct(data) = &input.data else {
        abort!(input, "expected struct");
    };
    
    for field in &data.fields {
        if !is_valid(field) {
            emit_error!(field, "invalid field");
        }
    }
    
    generate_code(&input).into()
}
```

Cleaner, right? And the second version reports *all* invalid fields, not just the first.

---

## Error Accumulation Pattern

The killer feature is showing multiple errors at once:

```rust
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Check all fields, accumulate errors
    for field in fields(&input) {
        if field.ty_is_unsupported() {
            emit_error!(field.ty, "unsupported type");
        }
        if field.missing_required_attr() {
            emit_error!(field, "missing required attribute");
        }
    }
    
    // If any errors were emitted, they're all shown here
    // If no errors, generate code
    generate_code(&input).into()
}
```

User sees:

```text
error: unsupported type
  --> src/main.rs:3:11
   |
3  |     count: HashMap<String, Vec<u8>>,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^

error: missing required attribute
  --> src/main.rs:4:5
   |
4  |     name: String,
   |     ^^^^

error: unsupported type
  --> src/main.rs:5:10
   |
5  |     data: [u8; 256],
   |           ^^^^^^^^^
```

Three errors, one compilation attempt. Much better UX!

---

## Integration with darling

darling also has great error handling. When to use which?

| Scenario | Use |
|----------|-----|
| Attribute parsing errors | darling's `write_errors()` |
| Validation after parsing | `emit_error!` / `abort!` |
| Combine both | Use darling first, then proc-macro-error2 |

### Combined Pattern

```rust
use proc_macro_error2::{proc_macro_error, abort, emit_error};
use darling::FromDeriveInput;

#[proc_macro_derive(MyMacro, attributes(my))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Let darling handle attribute parsing errors
    let parsed = match MyInput::from_derive_input(&input) {
        Ok(p) => p,
        Err(e) => {
            // darling's errors are already well-formatted
            return e.write_errors().into();
        }
    };
    
    // Use proc-macro-error2 for validation errors
    for field in &parsed.fields {
        if !field.is_valid() {
            emit_error!(field.span(), "validation failed");
        }
    }
    
    generate_code(&parsed).into()
}
```

---

## Adding Context to Errors

Make errors more helpful with additional context:

```rust
abort!(
    field.span(),
    "field `{}` cannot be optional", field.ident;
    help = "remove the Option<T> wrapper or add #[builder(default)]"
);
```

Output:
```text
error: field `config` cannot be optional
  --> src/main.rs:5:5
   |
5  |     config: Option<Config>,
   |     ^^^^^^
   |
   = help: remove the Option<T> wrapper or add #[builder(default)]
```

---

## Key Takeaways

📌 **`#[proc_macro_error]` on every entry point** — It's the gateway to clean error handling.

📌 **`abort!` for fatal errors** — When there's no point continuing.

📌 **`emit_error!` for accumulation** — Report multiple errors at once.

📌 **Use `proc-macro-error2`, not the original** — The `2` version is maintained.

📌 **Combine with darling** — Use darling for parsing, proc-macro-error2 for validation.

📌 **Better errors = happier users** — Worth the small dependency.

---

## Try It Yourself

The example in [`examples/06_error_handling/`](./examples/06_error_handling/) shows error patterns:

```bash
cd docs/proc_macro_tutorial/examples/06_error_handling
cargo run --example demo 2>&1
```

---

## Next Up

You've now seen all the pieces:
- `proc-macro` / `proc-macro2` for tokens
- `syn` for parsing
- `quote` for generating
- `darling` for attributes
- `heck` for case conversion
- `proc-macro-error2` for error handling

Time to see how they all fit together in a complete macro!

**[Continue to Chapter 7: The Complete Pipeline →](./07-pipeline.md)**

---

*[← Previous: Case Conversion with heck](./05-heck.md)*

