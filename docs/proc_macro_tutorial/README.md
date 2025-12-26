# The Hitchhiker's Guide to Procedural Macros 🚀

**Welcome, brave Rustacean!** You've decided to venture into the mystical realm of procedural macros—where code writes code, compile times fear to tread, and the humble `TokenStream` reigns supreme.

This tutorial will take you from "what even is a proc-macro?" to "I just wrote a derive macro that generates 500 lines of boilerplate, and I only needed 50 lines of code." That's not magic—that's macros.

## What You'll Learn

By the end of this tutorial, you'll understand how 7 crates work together to make procedural macros not just possible, but *pleasant*:

| Crate | Purpose | Chapter |
|-------|---------|---------|
| `proc-macro` | The compiler's gateway—where tokens enter and exit | [Chapter 1](./01-tokens.md) |
| `proc-macro2` | The testing-friendly twin | [Chapter 1](./01-tokens.md) |
| `syn` | Token chaos → structured AST | [Chapter 2](./02-syn.md) |
| `quote` | Your code → tokens (the reverse journey) | [Chapter 3](./03-quote.md) |
| `darling` | Attribute parsing without the pain | [Chapter 4](./04-darling.md) |
| `heck` | `snake_case` ↔ `PascalCase` made trivial | [Chapter 5](./05-heck.md) |
| `proc-macro-error2` | Error handling that doesn't make you cry | [Chapter 6](./06-errors.md) |

## Chapters

1. **[Understanding Tokens](./01-tokens.md)** — The foundation. Macros see tokens, not text.
2. **[Parsing with syn](./02-syn.md)** — Transform token soup into typed structures.
3. **[Generating Code with quote](./03-quote.md)** — Write Rust that writes Rust.
4. **[Ergonomic Attributes with darling](./04-darling.md)** — Parse `#[my_macro(thing = "value")]` without tears.
5. **[Case Conversion with heck](./05-heck.md)** — `user_name` → `UserName` → `USER_NAME` in one line.
6. **[Error Handling with proc-macro-error2](./06-errors.md)** — Because `abort!` beats 5 lines of boilerplate.
7. **[The Complete Pipeline](./07-pipeline.md)** — How it all fits together.

## How to Read This Tutorial

### 🚶 Linear Path (Recommended for Beginners)

Read chapters 1 → 2 → 3 first. They build on each other:
- **Chapter 1** teaches you what macros *see*
- **Chapter 2** teaches you how to *understand* that input
- **Chapter 3** teaches you how to *generate* output

After that, chapters 4, 5, and 6 can be read in any order—they're all "quality of life" improvements.

Chapter 7 ties everything together.

### 🏃 Skip-Ahead Path (For the Impatient)

Already know `TokenStream`? Jump to [Chapter 2](./02-syn.md).

Just want a reference? Go straight to [Chapter 7](./07-pipeline.md) for the complete picture.

Need to parse attributes? [Chapter 4](./04-darling.md) is your friend.

## Running the Examples

Each chapter has runnable examples in the [`examples/`](./examples/) directory.

### Prerequisites

```bash
# Install cargo-expand for viewing generated code
cargo install cargo-expand

# Ensure you have nightly (required by cargo-expand)
rustup install nightly
```

### Running an Example

```bash
# Navigate to an example
cd docs/proc_macro_tutorial/examples/02_parse_struct

# See the generated code
cargo +nightly expand

# Run the example to see eprintln! debugging output
cargo run --example demo
```

### 💡 Pro Tip: Debug with eprintln!

All our example macros use `eprintln!` to print debug info during compilation. To see this output:

```bash
cargo build 2>&1 | head -50
```

The `2>&1` captures stderr where `eprintln!` writes. This is your best friend for understanding what your macro sees.

## Crate Versions

This tutorial uses the same versions as the proc-macro-workshop:

```toml
[dependencies]
syn = { version = "2", features = ["derive", "parsing"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
proc-macro-error2 = "2"
heck = "0.5"
```

## Prerequisites

- Basic Rust knowledge (structs, enums, traits, generics)
- No prior proc-macro experience required
- Curiosity and a willingness to think about code that writes code

## Let's Go! 🎯

Ready? [Start with Chapter 1: Understanding Tokens →](./01-tokens.md)

---

*"Any sufficiently advanced macro is indistinguishable from magic." — Arthur C. Clarke (paraphrased)*

