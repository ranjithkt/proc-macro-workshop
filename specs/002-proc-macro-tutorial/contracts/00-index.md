# Contract: Tutorial Index (README.md)

## Purpose
Provide navigation and overview for the entire tutorial.

## Content Requirements

### What You'll Learn
- Overview of the proc-macro ecosystem
- Which crates to use and when
- Reading order recommendations

### Sections

1. **Introduction** (~100 words)
   - What are procedural macros?
   - Why this tutorial exists
   - Prerequisites (basic Rust)

2. **The Crate Ecosystem** (table)
   | Crate | Purpose | Chapter |
   |-------|---------|---------|
   | proc-macro | Compiler interface | 01 |
   | proc-macro2 | Testing & interop | 01 |
   | syn | Parsing | 02 |
   | quote | Code generation | 03 |
   | darling | Attribute parsing | 04 |
   | heck | Case conversion | 05 |

3. **Chapter List** (linked)
   - Chapter 1: Understanding Tokens
   - Chapter 2: Parsing with syn
   - Chapter 3: Generating Code with quote
   - Chapter 4: Ergonomic Attributes with darling
   - Chapter 5: Case Conversion with heck
   - Chapter 6: The Complete Pipeline

4. **How to Read This Tutorial**
   - Linear reading recommended for beginners
   - Chapter 4 and 5 can be read in either order
   - Code examples are runnable

5. **Running the Examples**
   - Instructions for `cargo expand`
   - Debug with `eprintln!`

## Estimated Length
~500 words

## Dependencies
None (entry point)

