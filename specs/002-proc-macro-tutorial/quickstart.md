# Quickstart: Proc-Macro Tutorial

## Overview

This tutorial teaches procedural macro development in Rust through 6 chapters covering the essential crates: proc-macro, proc-macro2, syn, quote, darling, and heck.

## Reading the Tutorial

### Linear Path (Recommended for Beginners)

```
README.md → 01-tokens.md → 02-syn.md → 03-quote.md → 04-darling.md → 05-heck.md → 06-pipeline.md
```

**Total time**: ~2 hours

### Skip-Ahead Path (For Experienced Rustaceans)

If you already know the basics:
- Skip to **02-syn.md** if you understand TokenStream
- Skip to **06-pipeline.md** for a reference walkthrough
- Use **04-darling.md** and **05-heck.md** as-needed references

## Running Code Examples

### Prerequisites

```bash
# Install cargo-expand for viewing generated code
cargo install cargo-expand

# Ensure you have nightly for cargo-expand
rustup install nightly
```

### Running an Example

Each chapter has examples in `docs/proc-macro-tutorial/examples/`:

```bash
# Navigate to an example
cd docs/proc-macro-tutorial/examples/02-parse-struct

# See the generated code
cargo +nightly expand

# Run tests
cargo test
```

### Debugging with eprintln!

All examples include `eprintln!` statements. To see debug output:

```bash
# Run with cargo test to see eprintln! output
cargo test -- --nocapture
```

## Chapter Structure

Each chapter follows this format:

1. **What You'll Learn** - Quick overview
2. **The Problem** - What pain point this solves
3. **The Solution** - How the crate addresses it
4. **Code Examples** - Runnable demonstrations
5. **Key Takeaways** - Summary bullets

## Diagram Viewing

Diagrams use Mermaid syntax. They render automatically in:
- ✅ GitHub
- ✅ VS Code (built-in markdown preview)
- ✅ GitLab
- ⚠️ Other editors may need plugins

## Quick Reference

### Crate Purposes

| Crate | One-Line Purpose |
|-------|------------------|
| proc-macro | Compiler interface for macros |
| proc-macro2 | Testing-friendly token types |
| syn | Parse tokens to AST |
| quote | Generate tokens from Rust-like syntax |
| darling | Declarative attribute parsing |
| heck | Case conversion utilities |

### Common Imports

```rust
// Derive macro essentials
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// With darling
use darling::{FromDeriveInput, FromField};

// With heck
use heck::{ToSnakeCase, ToPascalCase};
```

### Minimal Derive Macro Template

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyMacro)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    quote! {
        impl #name {
            fn hello() { println!("Hello from {}!", stringify!(#name)); }
        }
    }.into()
}
```

## Troubleshooting

### "proc-macro crate types cannot export any items"

Your lib.rs in a proc-macro crate should only have:
- `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]` functions
- No `pub mod`, `pub struct`, etc.

### "can't find crate for `proc_macro`"

Ensure your Cargo.toml has:
```toml
[lib]
proc-macro = true
```

### Debugging Tips

1. Use `eprintln!("{:#?}", input)` to inspect parsed structures
2. Use `cargo +nightly expand` to see generated code
3. Check `.stderr` files in trybuild tests for expected errors

## Next Steps

After completing the tutorial:
1. Try the [proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop) exercises
2. Read the [syn documentation](https://docs.rs/syn)
3. Explore real-world macros: serde_derive, thiserror, tokio::main

