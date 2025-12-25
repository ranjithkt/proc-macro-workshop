# Contract: Sorted Macro

**Project**: `sorted/`  
**Macro Type**: Attribute macro (two entry points)  
**Entry Points**: 
- `#[proc_macro_attribute] fn sorted`
- `#[proc_macro_attribute] fn check`

## Input Contract

### Valid Input: Sorted Enum

```rust
#[sorted]
pub enum Error {
    Fmt(fmt::Error),
    Io(io::Error),
    Parse(ParseError),
}
```

### Valid Input: Sorted Match Check

```rust
#[sorted::check]
fn handle_error(e: Error) {
    #[sorted]
    match e {
        Error::Fmt(e) => println!("fmt: {}", e),
        Error::Io(e) => println!("io: {}", e),
        Error::Parse(e) => println!("parse: {}", e),
    }
}
```

### Input Constraints

| Macro | Valid On | Purpose |
|-------|----------|---------|
| `#[sorted]` | enum | Validate variants are alphabetically sorted |
| `#[sorted]` | match (inside `#[sorted::check]` fn) | Validate arms are alphabetically sorted |
| `#[sorted::check]` | fn | Find and validate inner `#[sorted]` matches |

## Output Contract

### Passing Case

Input:
```rust
#[sorted]
pub enum Conference {
    RustBeltRust,
    RustConf,
    RustFest,
}
```

Output (unchanged):
```rust
pub enum Conference {
    RustBeltRust,
    RustConf,
    RustFest,
}
```

Note: `#[sorted]` is stripped but enum is returned unchanged.

### Failing Case: Unsorted Enum

Input:
```rust
#[sorted]
pub enum Error {
    Io(io::Error),
    Fmt(fmt::Error),  // Out of order!
}
```

Output (compile error):
```
error: Fmt should sort before Io
 --> tests/03-out-of-order.rs:8:5
  |
8 |     Fmt(fmt::Error),
  |     ^^^
```

### Failing Case: Not an Enum

Input:
```rust
#[sorted]
pub struct NotAnEnum {
    field: u32,
}
```

Output (compile error):
```
error: expected enum or match expression
 --> tests/02-not-enum.rs:3:1
  |
3 | pub struct NotAnEnum {
  | ^^^
```

### Match Expression Validation

Input:
```rust
#[sorted::check]
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    #[sorted]
    match self {
        Error::Io(e) => write!(f, "{}", e),
        Error::Fmt(e) => write!(f, "{}", e),  // Out of order!
    }
}
```

Output:
```
error: Fmt should sort before Io
  --> tests/05-match-expr.rs:88:13
   |
88 |             Fmt(e) => write!(f, "{}", e),
   |             ^^^
```

Note: The `#[sorted]` on match is stripped after checking.

## Sorting Rules

### Alphabetical Comparison

- Case-sensitive ASCII ordering
- Compare variant/pattern names as strings
- `Aaa` < `Bbb` < `aaa` (uppercase before lowercase)

### Pattern Extraction

For match arms, extract the sortable name:

| Pattern | Extracted Name |
|---------|----------------|
| `Error::Io(e)` | `Io` |
| `Error::Fmt(e)` | `Fmt` |
| `Io(e)` | `Io` |
| `_ ` | (wildcard, always valid at end) |

### Underscore Handling

The wildcard pattern `_` is valid only as the last arm and is not checked for sorting.

## Error Message Format

```
error: {found} should sort before {expected}
 --> {file}:{line}:{column}
  |
{line} |     {pattern_code}
  |     ^^^
```

- `found`: The variant/arm that is out of order
- `expected`: What it should come before
- Span points to the out-of-order item

## Test Coverage

| Test | Description | Type |
|------|-------------|------|
| 01-parse-enum.rs | Basic enum parsing | pass |
| 02-not-enum.rs | Error on non-enum | fail (expected) |
| 03-out-of-order.rs | Detect unsorted variants | fail (expected) |
| 04-variants-with-data.rs | Enums with associated data | fail (expected) |
| 05-match-expr.rs | Match expression checking | fail (expected) |
| 06-pattern-path.rs | Path patterns (Error::Io) | fail (expected) |
| 07-unrecognized-pattern.rs | Unknown patterns error | fail (expected) |
| 08-underscore.rs | Wildcard handling | pass |

