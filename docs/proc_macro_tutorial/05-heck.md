# Chapter 5: Case Conversion with heck 🔤

## What You'll Learn

- Why case conversion matters in macros
- heck's case conversion traits
- Common patterns for generated identifiers

---

## The Problem: Naming Generated Methods

You're writing a derive macro. The struct has a field `user_name`, and you need to generate:

- A getter: `get_user_name()`
- A setter: `set_user_name()`
- A builder method: `with_user_name()`
- Maybe an environment variable: `USER_NAME`

Or you have a field `args` and need the "each" setter to be `arg` (singular).

You *could* do string manipulation manually:

```rust
// 😬 Manual case conversion
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
```

But why reinvent the wheel? **Enter `heck`**: a tiny crate that handles all case conversions correctly.

---

## heck's Case Conversion Traits

`heck` provides traits for each case style:

| Trait | Input | Output |
|-------|-------|--------|
| `ToSnakeCase` | `"UserName"` | `"user_name"` |
| `ToPascalCase` | `"user_name"` | `"UserName"` |
| `ToLowerCamelCase` | `"user_name"` | `"userName"` |
| `ToKebabCase` | `"UserName"` | `"user-name"` |
| `ToShoutySnakeCase` | `"userName"` | `"USER_NAME"` |
| `ToTitleCase` | `"user_name"` | `"User Name"` |
| `ToTrainCase` | `"user_name"` | `"User-Name"` |

### Basic Usage

```rust
use heck::{ToSnakeCase, ToPascalCase, ToShoutySnakeCase};

let field_name = "UserEmail";

let snake = field_name.to_snake_case();     // "user_email"
let pascal = field_name.to_pascal_case();   // "UserEmail" 
let shouty = field_name.to_shouty_snake_case(); // "USER_EMAIL"
```

**💡 Aha!** These traits work on `&str` and `String`—and they handle edge cases like acronyms (`XMLParser` → `xml_parser`) correctly.

---

## Common Patterns

### Pattern 1: Getter/Setter Methods

```rust
use heck::ToSnakeCase;
use quote::format_ident;

fn generate_getter(field_name: &syn::Ident) -> proc_macro2::TokenStream {
    let field_str = field_name.to_string();
    let getter_name = format_ident!("get_{}", field_str.to_snake_case());
    
    quote::quote! {
        pub fn #getter_name(&self) -> &Self::#field_name {
            &self.#field_name
        }
    }
}
```

### Pattern 2: Builder Struct Names

```rust
use heck::ToPascalCase;
use quote::format_ident;

let struct_name = "my_config";
let builder_name = format_ident!("{}Builder", struct_name.to_pascal_case());
// MyConfigBuilder
```

### Pattern 3: Environment Variables

```rust
use heck::ToShoutySnakeCase;

let field_name = "databaseUrl";
let env_var = format!("APP_{}", field_name.to_shouty_snake_case());
// APP_DATABASE_URL
```

### Pattern 4: CLI Arguments

```rust
use heck::ToKebabCase;

let field_name = "outputFile";
let cli_arg = format!("--{}", field_name.to_kebab_case());
// --output-file
```

---

## Combining with format_ident!

When you need the result as an `Ident` (not a string), combine with `format_ident!`:

```rust
use heck::ToSnakeCase;
use quote::format_ident;

// Field: userName
// Generated method: with_user_name

let field_name: &syn::Ident = /* from parsed input */;
let field_str = field_name.to_string();

let method_name = format_ident!(
    "with_{}",
    field_str.to_snake_case()
);

quote::quote! {
    pub fn #method_name(&mut self, value: String) -> &mut Self {
        self.#field_name = value;
        self
    }
}
```

### Preserving Spans

For better error messages, pass the original span:

```rust
let method_name = format_ident!(
    "with_{}",
    field_str.to_snake_case(),
    span = field_name.span()
);
```

---

## Real Example: Enum Variants to Strings

Generating string representations for enum variants:

```rust
use heck::{ToKebabCase, ToShoutySnakeCase};
use quote::quote;

fn generate_to_string(variants: &[syn::Variant]) -> proc_macro2::TokenStream {
    let arms = variants.iter().map(|v| {
        let name = &v.ident;
        let kebab = name.to_string().to_kebab_case();
        
        quote! {
            Self::#name => #kebab
        }
    });
    
    quote! {
        pub fn as_str(&self) -> &'static str {
            match self {
                #( #arms, )*
            }
        }
    }
}
```

For `enum Status { InProgress, Completed, Failed }`:

```rust
pub fn as_str(&self) -> &'static str {
    match self {
        Self::InProgress => "in-progress",
        Self::Completed => "completed",
        Self::Failed => "failed",
    }
}
```

---

## The Workshop's Usage

In the Builder macro, heck is used for creating builder type names:

```rust
use heck::ToPascalCase;
use quote::format_ident;

// If struct is "command", builder should be "CommandBuilder"
let builder_ident = format_ident!(
    "{}Builder",
    struct_name.to_string().to_pascal_case()
);
```

This ensures consistent naming regardless of how the user writes their struct name.

---

## Key Takeaways

📌 **heck provides simple case conversion** — No need to implement it yourself.

📌 **Use traits on strings** — `ToSnakeCase`, `ToPascalCase`, etc.

📌 **Combine with `format_ident!`** — For creating identifiers from converted names.

📌 **Common uses**: method names, env vars, CLI args, type names.

📌 **Handles edge cases** — Acronyms, numbers, unicode.

---

## Try It Yourself

The example in [`examples/05_case_convert/`](./examples/05_case_convert/) demonstrates case conversion:

```bash
cd docs/proc_macro_tutorial/examples/05_case_convert
cargo run --example demo 2>&1
```

---

## Next Up

You've now got the complete toolkit for writing derive macros:
- `syn` for parsing
- `quote` for generating
- `darling` for attributes
- `heck` for case conversion

But what about when things go wrong? How do you report errors to users without writing the same boilerplate over and over?

That's where `proc-macro-error2` comes in—ergonomic error handling for macros.

**[Continue to Chapter 6: Error Handling with proc-macro-error2 →](./06-errors.md)**

---

*[← Previous: Ergonomic Attributes with darling](./04-darling.md)*

