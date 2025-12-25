# Contract: CustomDebug Macro

**Project**: `debug/`  
**Macro Type**: Derive macro  
**Entry Point**: `#[proc_macro_derive(CustomDebug, attributes(debug))]`

## Input Contract

### Valid Input

```rust
#[derive(CustomDebug)]
pub struct Field {
    name: String,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

#[derive(CustomDebug)]
#[debug(bound = "T::Value: Debug")]
pub struct Wrapper<T: Trait> {
    field: Field<T>,
}
```

### Input Constraints

| Constraint | Requirement |
|------------|-------------|
| Item type | Named struct only |
| Field attribute | `#[debug = "format_string"]` for custom formatting |
| Struct attribute | `#[debug(bound = "where_predicate")]` for custom bounds |
| Generics | Fully supported with intelligent bound inference |

## Output Contract

### Basic Debug Implementation

For input:
```rust
#[derive(CustomDebug)]
pub struct Field {
    name: String,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}
```

Output:
```rust
impl ::std::fmt::Debug for Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Field")
            .field("name", &self.name)
            .field("bitmask", &::std::format_args!("0b{:08b}", self.bitmask))
            .finish()
    }
}
```

### Generic with Inferred Bounds

For input:
```rust
#[derive(CustomDebug)]
pub struct Wrapper<T> {
    field: T,
}
```

Output:
```rust
impl<T: ::std::fmt::Debug> ::std::fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Wrapper")
            .field("field", &self.field)
            .finish()
    }
}
```

### PhantomData Handling

For input:
```rust
#[derive(CustomDebug)]
pub struct Wrapper<T> {
    marker: PhantomData<T>,
    value: String,
}
```

Output (note: NO `T: Debug` bound):
```rust
impl<T> ::std::fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Wrapper")
            .field("marker", &self.marker)
            .field("value", &self.value)
            .finish()
    }
}
```

### Custom Bound Escape Hatch

For input:
```rust
#[derive(CustomDebug)]
#[debug(bound = "T::Value: Debug")]
pub struct Wrapper<T: Trait> {
    field: Field<T>,
}
```

Output:
```rust
impl<T: Trait> ::std::fmt::Debug for Wrapper<T>
where
    T::Value: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Wrapper")
            .field("field", &self.field)
            .finish()
    }
}
```

## Bound Inference Rules

1. **Direct usage**: `field: T` → add `T: Debug`
2. **Wrapped usage**: `field: Option<T>` → add `T: Debug`
3. **PhantomData**: `field: PhantomData<T>` → NO bound added
4. **Associated types**: `field: T::Item` → need escape hatch
5. **Escape hatch**: `#[debug(bound = "...")]` replaces ALL inferred bounds

## Test Coverage

| Test | Description | Type |
|------|-------------|------|
| 01-parse.rs | Macro exists and parses input | pass |
| 02-impl-debug.rs | Basic Debug impl generated | pass |
| 03-custom-format.rs | #[debug = "..."] formatting | pass |
| 04-type-parameter.rs | Generic bounds inferred | pass |
| 05-phantom-data.rs | PhantomData skipped | pass |
| 06-bound-trouble.rs | Associated type inference issues | pass |
| 07-associated-type.rs | More associated type handling | pass |
| 08-escape-hatch.rs | #[debug(bound = "...")] | pass |

