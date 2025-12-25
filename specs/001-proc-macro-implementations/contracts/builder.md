# Contract: Builder Macro

**Project**: `builder/`  
**Macro Type**: Derive macro  
**Entry Point**: `#[proc_macro_derive(Builder, attributes(builder))]`

## Input Contract

### Valid Input

```rust
#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}
```

### Input Constraints

| Constraint | Requirement |
|------------|-------------|
| Item type | Named struct only (not tuple struct, not enum) |
| Field types | Any type; `Option<T>` treated as optional |
| Attribute format | `#[builder(each = "literal_string")]` |
| Generics | Supported (though not tested extensively) |

### Invalid Input (compile errors expected)

```rust
#[derive(Builder)]
pub struct Bad {
    #[builder(eac = "typo")]  // Unrecognized attribute key
    args: Vec<String>,
}
```

## Output Contract

### Generated Types

For input `struct Command { ... }`:

```rust
// Builder struct with all fields wrapped in Option
pub struct CommandBuilder {
    executable: ::std::option::Option<String>,
    args: ::std::option::Option<::std::vec::Vec<String>>,
    env: ::std::option::Option<::std::vec::Vec<String>>,
    current_dir: ::std::option::Option<String>,
}
```

### Generated Methods

```rust
impl Command {
    /// Create a new builder for Command
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: ::std::option::Option::None,
            args: ::std::option::Option::None,
            env: ::std::option::Option::None,
            current_dir: ::std::option::Option::None,
        }
    }
}

impl CommandBuilder {
    /// Set the executable field
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = ::std::option::Option::Some(executable);
        self
    }

    /// Add a single arg (from #[builder(each = "arg")])
    pub fn arg(&mut self, arg: String) -> &mut Self {
        self.args.get_or_insert_with(::std::vec::Vec::new).push(arg);
        self
    }

    /// Set all args at once
    pub fn args(&mut self, args: ::std::vec::Vec<String>) -> &mut Self {
        self.args = ::std::option::Option::Some(args);
        self
    }

    // ... similar for env, current_dir ...

    /// Build the Command, returning error if required fields missing
    pub fn build(&mut self) -> ::std::result::Result<Command, ::std::boxed::Box<dyn ::std::error::Error>> {
        ::std::result::Result::Ok(Command {
            executable: self.executable.clone()
                .ok_or("executable is required")?,
            args: self.args.clone()
                .unwrap_or_else(::std::vec::Vec::new),
            env: self.env.clone()
                .unwrap_or_else(::std::vec::Vec::new),
            current_dir: self.current_dir.clone(),  // Option stays Option
        })
    }
}
```

## Error Messages

### Unrecognized Attribute

**Input**:
```rust
#[builder(eac = "arg")]
```

**Output** (compile error):
```
error: expected `builder(each = "...")`
 --> tests/08-unrecognized-attribute.rs:22:7
  |
22|     #[builder(eac = "arg")]
  |       ^^^^^^^
```

## Test Coverage

| Test | Description | Type |
|------|-------------|------|
| 01-parse.rs | Macro exists and parses input | pass |
| 02-create-builder.rs | Builder struct generated | pass |
| 03-call-setters.rs | Setter methods work | pass |
| 04-call-build.rs | build() method works | pass |
| 05-method-chaining.rs | Setters return &mut Self | pass |
| 06-optional-field.rs | Option<T> fields optional | pass |
| 07-repeated-field.rs | #[builder(each)] works | pass |
| 08-unrecognized-attribute.rs | Bad attributes error | fail (expected) |
| 09-redefined-prelude-types.rs | Fully qualified paths | pass |

