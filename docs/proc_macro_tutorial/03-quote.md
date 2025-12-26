# Chapter 3: Code Generation with quote 📝

## What You'll Learn

- The `quote!` macro for generating code
- Variable interpolation with `#`
- Repetition with `#(...)*`
- Preserving spans with `quote_spanned!`
- Creating identifiers with `format_ident!`

---

## The Problem: Building TokenStreams Manually

After Chapter 2, you can parse Rust code into structured types. But a macro needs to *return* a `TokenStream`—newly generated code. How do you build that?

You *could* construct tokens manually:

```rust
// 😱 The Hard Way™
use proc_macro2::{TokenStream, TokenTree, Ident, Punct, Spacing, Span};

fn generate_impl_manually(name: &Ident) -> TokenStream {
    let mut tokens = TokenStream::new();
    tokens.extend([
        TokenTree::Ident(Ident::new("impl", Span::call_site())),
        TokenTree::Ident(name.clone()),
        TokenTree::Punct(Punct::new('{', Spacing::Alone)),
        // ... 50 more lines of this nightmare
    ]);
    tokens
}
```

That's like writing HTML by concatenating strings. There has to be a better way.

**Enter `quote`**: write Rust-like syntax, get `TokenStream` out.

---

## Basic quote! Usage

The `quote!` macro lets you write code that looks like regular Rust:

```rust
use quote::quote;

let name = quote::format_ident!("Foo");

let tokens = quote! {
    impl #name {
        fn hello() {
            println!("Hello from {}!", stringify!(#name));
        }
    }
};
```

This generates a `TokenStream` (from `proc_macro2`) that, when output, becomes:

```rust
impl Foo {
    fn hello() {
        println!("Hello from {}!", stringify!(Foo));
    }
}
```

**💡 Aha!** The `#name` is *interpolation*—it inserts the value of the `name` variable. Everything else is literal tokens.

---

## Variable Interpolation

The `#` sigil is your friend:

```rust
use quote::quote;
use syn::Ident;

fn generate_getter(field_name: &Ident, field_type: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        pub fn #field_name(&self) -> &#field_type {
            &self.#field_name
        }
    }
}
```

### What Can You Interpolate?

Anything that implements `ToTokens`:

| Type | What It Produces |
|------|-----------------|
| `Ident` | An identifier token |
| `syn::Type` | A type expression |
| `syn::Expr` | An expression |
| `TokenStream` | Embedded tokens |
| `&str` / `String` | Via `quote::format_ident!` |
| Literals | Via `quote::quote!` |

### Interpolating Types

```rust
let ty = &field.ty;  // Some syn::Type

quote! {
    fn get_value() -> #ty {
        // #ty becomes the actual type, like String or Vec<i32>
        todo!()
    }
}
```

---

## Repetition Syntax

The real power of `quote!` is handling collections. The `#(...)*` syntax iterates:

```rust
use quote::quote;

let field_names = vec![
    quote::format_ident!("name"),
    quote::format_ident!("age"),
    quote::format_ident!("email"),
];

let tokens = quote! {
    struct Generated {
        #( #field_names: String, )*
    }
};
```

This generates:

```rust
struct Generated {
    name: String,
    age: String,
    email: String,
}
```

### With Separators

Use `#(...),*` to add separators between items:

```rust
let values = vec![1i32, 2, 3, 4, 5];

quote! {
    let array = [#( #values ),*];
}
```

Generates:

```rust
let array = [1, 2, 3, 4, 5];
```

### Multiple Variables in Repetition

You can interpolate multiple variables of the same length:

```rust
let names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

quote! {
    #(
        pub fn #names(&self) -> &#types {
            &self.#names
        }
    )*
}
```

Each iteration uses the corresponding element from both `names` and `types`.

### Real Example: Builder Pattern

```rust
// For each field, generate a setter method
let setters = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    quote! {
        pub fn #name(&mut self, value: #ty) -> &mut Self {
            self.#name = Some(value);
            self
        }
    }
});

quote! {
    impl Builder {
        #( #setters )*
    }
}
```

---

## quote_spanned! for Better Errors

When you generate code, the compiler needs to know *where* errors should point. By default, generated code points to the macro call site—not very helpful.

`quote_spanned!` lets you specify exactly where errors should appear:

```rust
use quote::quote_spanned;
use syn::spanned::Spanned;

fn validate_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let span = field.span();  // Get the span of this field
    let ty = &field.ty;
    
    quote_spanned! { span =>
        // If this trait bound fails, the error points to the FIELD,
        // not to wherever the macro was called
        const _: () = {
            fn assert_debug<T: std::fmt::Debug>() {}
            assert_debug::<#ty>();
        };
    }
}
```

**💡 Aha!** Without `quote_spanned!`, the user would see "trait bound not satisfied" pointing at `#[derive(MyMacro)]`. With it, they see the error pointing at the specific field that lacks `Debug`.

### When to Use quote_spanned!

- **Trait bound checks**: Point to the type that needs the trait
- **Validation errors**: Point to the invalid input
- **Generated method calls**: Point to the field being accessed

---

## format_ident! for Dynamic Names

Sometimes you need to create identifiers from strings:

```rust
use quote::format_ident;

let base = "user";
let getter = format_ident!("get_{}", base);      // get_user
let setter = format_ident!("set_{}", base);      // set_user
let builder = format_ident!("{}Builder", base);  // userBuilder
```

### Combining with Existing Idents

```rust
let field_name: &Ident = &field.ident;
let with_method = format_ident!("with_{}", field_name);

quote! {
    fn #with_method(&mut self, value: String) -> &mut Self {
        self.#field_name = value;
        self
    }
}
```

### Preserving Spans

To keep error messages helpful, pass a span:

```rust
let span = field_name.span();
let getter = format_ident!("get_{}", field_name, span = span);
```

---

## Putting It Together: A Simple Debug Derive

Here's a complete example that generates a `Debug` implementation:

```rust
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(SimpleDebug)]
pub fn derive_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let debug_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_debugs = fields.named.iter().map(|f| {
                    let fname = f.ident.as_ref().unwrap();
                    let fname_str = fname.to_string();
                    quote! {
                        .field(#fname_str, &self.#fname)
                    }
                });
                quote! { #( #field_debugs )* }
            }
            _ => quote! {},
        },
        _ => quote! {},
    };
    
    let name_str = name.to_string();
    
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#name_str)
                    #debug_fields
                    .finish()
            }
        }
    };
    
    eprintln!("Generated code:\n{}", expanded);
    
    expanded.into()
}
```

For a struct like:

```rust
#[derive(SimpleDebug)]
struct User {
    name: String,
    age: u32,
}
```

This generates:

```rust
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}
```

---

## Key Takeaways

📌 **`quote!` generates TokenStream from Rust-like syntax** — Write code that writes code.

📌 **`#var` interpolates variables** — Anything implementing `ToTokens` works.

📌 **`#(...)*` for repetition** — Iterate over collections cleanly.

📌 **`#(...),*` adds separators** — Put commas, semicolons, etc. between items.

📌 **`quote_spanned!` preserves error locations** — Point errors to the right source.

📌 **`format_ident!` creates identifiers** — Build method names dynamically.

---

## Try It Yourself

The example in [`examples/03_generate_impl/`](./examples/03_generate_impl/) shows code generation:

```bash
cd docs/proc_macro_tutorial/examples/03_generate_impl
cargo run --example demo 2>&1

# See the generated code with cargo-expand:
cargo +nightly expand --example demo
```

---

## Next Up

You've mastered the core trio: `TokenStream` → `syn` → `quote`. That's enough to write most macros!

But have you noticed how parsing attributes gets verbose? In Chapter 4, we'll meet `darling`—the crate that turns 30 lines of attribute parsing into 5.

**[Continue to Chapter 4: Ergonomic Attributes with darling →](./04-darling.md)**

---

*[← Previous: Parsing with syn](./02-syn.md)*

