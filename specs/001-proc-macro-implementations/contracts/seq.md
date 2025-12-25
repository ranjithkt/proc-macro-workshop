# Contract: Seq Macro

**Project**: `seq/`  
**Macro Type**: Function-like procedural macro  
**Entry Point**: `#[proc_macro]`

## Input Contract

### Valid Input

```rust
seq!(N in 0..8 {
    // Body tokens
});

seq!(N in 0..=16 {
    struct Cpu~N;
});

seq!(N in 0..16 {
    #[derive(Debug)]
    enum Interrupt {
        #(
            Irq~N,
        )*
    }
});
```

### Input Syntax

```text
seq!( IDENT in RANGE { BODY })

IDENT  := Any valid Rust identifier (typically N, I, etc.)
RANGE  := LitInt .. LitInt        (exclusive end)
        | LitInt ..= LitInt       (inclusive end)
BODY   := Any tokens, including:
          - IDENT~IDENT for pasting
          - #( ... )* for repetition
```

### Input Constraints

| Constraint | Requirement |
|------------|-------------|
| Range values | Non-negative integer literals |
| Range order | Start <= End |
| Body | Any valid tokens; will be repeated/pasted |
| Pasting | `Prefix~N` where N is the loop variable |
| Repetition | `#(...)* ` marks section to repeat |

## Output Contract

### Simple Expansion

Input:
```rust
seq!(N in 0..3 {
    struct S~N;
});
```

Output:
```rust
struct S0;
struct S1;
struct S2;
```

### Identifier Pasting

Input:
```rust
seq!(N in 0..3 {
    fn make~N() -> u32 { N }
});
```

Output:
```rust
fn make0() -> u32 { 0 }
fn make1() -> u32 { 1 }
fn make2() -> u32 { 2 }
```

### Selective Repetition

Input:
```rust
seq!(N in 0..4 {
    enum Direction {
        #(
            D~N,
        )*
    }
});
```

Output:
```rust
enum Direction {
    D0,
    D1,
    D2,
    D3,
}
```

Note: The `enum Direction { }` wrapper appears once; only `D~N,` is repeated.

### Inclusive Range

Input:
```rust
seq!(N in 0..=2 {
    const C~N: u8 = N;
});
```

Output:
```rust
const C0: u8 = 0;
const C1: u8 = 1;
const C2: u8 = 2;
```

## Token Processing Rules

### Pasting (`~`)

1. Scan token stream for `Ident ~ Ident` sequence
2. If second ident matches loop variable, paste first ident with current value
3. Resulting ident gets span from first ident (for error messages)

### Repetition (`#(...)*`)

1. Find `# ( ... ) *` sequences in body
2. Extract tokens inside parentheses
3. Repeat those tokens for each iteration
4. Tokens outside `#(...)*` appear exactly once

### Bare Variable Usage

The loop variable `N` appearing alone (not in paste) expands to the literal number:

```rust
seq!(N in 0..3 {
    let x = N;  // becomes: let x = 0; let x = 1; let x = 2;
});
```

## Error Messages

### Invalid Range

**Input**:
```rust
seq!(N in 10..5 { });
```

**Output**:
```
error: range start must be <= end
 --> src/main.rs:1:10
  |
1 | seq!(N in 10..5 { });
  |          ^^
```

## Test Coverage

| Test | Description | Type |
|------|-------------|------|
| 01-parse-header.rs | Header parsing | pass |
| 02-parse-body.rs | Body parsing | pass |
| 03-expand-four-errors.rs | Error span handling | fail (expected) |
| 04-paste-ident.rs | Identifier pasting | pass |
| 05-repeat-section.rs | #(...)* repetition | pass |
| 06-init-array.rs | Array initialization | pass |
| 07-inclusive-range.rs | ..= syntax | pass |
| 08-ident-span.rs | Span correctness | fail (expected) |
| 09-interaction-with-macrorules.rs | Works with macro_rules! | pass |

