# Chapter 4: Ergonomic Attributes with darling 🌹

## What You'll Learn

- Why manual attribute parsing is painful
- How darling derives parsers from structs
- `FromDeriveInput`, `FromField`, `FromMeta` traits
- Handling optional attributes and defaults

---

## The Goal: A `#[derive(Builder)]` Macro

Before we talk about attribute parsing, let's be clear about *what we're building*.

Consider this struct:

```rust
pub struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: Option<String>,
}
```

Creating a `Command` directly is a bit clunky — you have to provide all fields at once:

```rust
let cmd = Command {
    executable: "cargo".to_owned(),
    args: vec!["build".to_owned(), "--release".to_owned()],
    current_dir: None,
};
```

The **Builder pattern** solves this: instead of constructing the struct directly, you call chainable setter methods and finish with `.build()`. Much nicer:

```rust
let cmd = Command::builder()
    .executable("cargo".to_owned())
    .arg("build".to_owned())        // add one arg at a time!
    .arg("--release".to_owned())
    .build()
    .unwrap();
```

Writing builder boilerplate by hand is tedious. So we want a **derive macro** that generates it for us:

```rust
#[derive(Builder)]   // ← our macro!
pub struct Command {
    executable: String,
    #[builder(each = "arg")]   // ← helper attribute: "generate a method named `arg`"
    args: Vec<String>,
    current_dir: Option<String>,
}
```

The `#[builder(each = "arg")]` is a **helper attribute**. It tells our macro: "for this `Vec` field, generate a method named `arg` that pushes one element at a time."

**Our macro's job:**

1. Parse `Command` and its fields.
2. Read any `#[builder(...)]` helper attributes.
3. Generate a `CommandBuilder` struct with setters and a `build()` method.

Steps 1 and 3 are straightforward with `syn` and `quote`. But step 2 — reading helper attributes — is where things get *messy*.

---

## The Problem: Attribute Parsing Spaghetti

So… how does our derive macro *read* `#[builder(each = "arg")]` and turn it into a method named `arg`?

### Where do these `Attribute`s come from?

In a derive macro, you start with a `TokenStream`, parse it into a `syn::DeriveInput`, and then poke around inside that AST. `syn` represents *every* `#[...]` as an `Attribute`, and you’ll see them in two main places:

- **On the container** (the struct/enum itself): `input.attrs`
- **On each field**: `field.attrs`

So for code like:

```rust
#[derive(Builder)]
struct Command {
    #[builder(each = "arg")]     // field attribute (goes on syn::Field)
    args: Vec<String>,
}
```

`syn` hands your macro a tree that (very roughly) looks like:

```text
DeriveInput
├─ attrs: Vec<Attribute>         // attributes on the item (struct/enum)
└─ data: Data::Struct
   └─ fields: Vec<Field>
      └─ Field.attrs: Vec<Attribute>   // attributes on each field
```

And each `Attribute` contains a parsed “meta item” — the thing inside the brackets:

- `#[builder]` → `Meta::Path`
- `#[builder = "..."]` → `Meta::NameValue` (rare for helpers, but possible)
- `#[builder(each = "arg")]` → `Meta::List`

That’s why manual parsing turns into nested `match` soup so quickly: you’re walking this meta structure by hand.

Even for *one simple key* like `each`, this gets verbose fast. Now imagine parsing:

```rust
#[builder(each = "arg", default = true, rename = "something")]
```

You'd need 50+ lines of deeply nested pattern matching. And error messages? You'd have to craft them manually for every case.

**💡 Aha!** What if we could just define a struct and have the parsing generated automatically? That's `darling`.

But first, let's see exactly how painful the manual approach is — so you'll *really* appreciate what darling does.

---

## The old way: manual parsing (a.k.a. "attribute lasagna")

Let's write the attribute-parsing code the hard way, using just `syn`.

At the top level, every derive macro begins the same way:

```rust
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let parsed = match old_way_parse(&input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    // Now use `parsed` to drive quote! code generation…
    todo!()
}
```

One important detail: the `attributes(builder)` part is not decoration.

It tells the compiler that `#[builder(...)]` is an *inert helper attribute* associated with this derive macro. Without it, Rust rejects the caller’s code as “unknown attribute” before your macro even gets a chance to run.

So what’s inside `old_way_parse`? A whole lot of “find `#[builder(...)]`, parse its nested meta, validate, repeat”:

```rust
struct OldFieldConfig {
    each: Option<String>,
}

fn old_way_parse(input: &DeriveInput) -> syn::Result<Vec<OldFieldConfig>> {
    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(named) => &named.named,
            _ => return Err(syn::Error::new_spanned(&s.fields, "expected named fields")),
        },
        _ => return Err(syn::Error::new_spanned(input, "Builder only supports structs")),
    };

    fields.iter().map(old_way_parse_field).collect()
}

fn old_way_parse_field(field: &syn::Field) -> syn::Result<OldFieldConfig> {
    use syn::{punctuated::Punctuated, Expr, ExprLit, Lit, Meta, Token};

    for attr in &field.attrs {
        if !attr.path().is_ident("builder") {
            continue;
        }

        let Meta::List(list) = &attr.meta else {
            return Err(syn::Error::new_spanned(attr, "expected #[builder(...)]"));
        };

        // Parses `each = "arg", default = true, ...` into a list of Meta items
        let nested = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("each") {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                        return Ok(OldFieldConfig { each: Some(s.value()) });
                    }
                    return Err(syn::Error::new_spanned(nv.value, "each must be a string literal"));
                }
            }
        }
    }

    Ok(OldFieldConfig { each: None })
}
```

That is… a lot of code to extract one tiny `Option<String>`. And we haven't even started generating the builder yet.

Let's recap what this manual approach requires:

- Match `Data::Struct` → `Fields::Named` to get fields
- Loop through each field's `attrs`
- Check if the attribute path is `builder`
- Parse `Meta::List` contents into `Punctuated<Meta, Token![,]>`
- Match each nested `Meta::NameValue`
- Extract the string literal from `Expr::Lit`
- Craft error messages manually for every failure case

That's 6+ layers of pattern matching for *one attribute key*. Now imagine adding `default`, `rename`, `skip`… 😱

---

## The new way: darling (a.k.a. "serde, but for attributes")

Time for the good stuff.

`darling` is best thought of as **a translator from `syn`'s AST into your own Rust structs**. Instead of manually spelunking through `Meta::List`, you *describe* what you want to extract — and darling generates the parsing code for you.

### Struct-level parsing with `FromDeriveInput`

```rust
use darling::{FromDeriveInput, ast::Data};
use syn::{DeriveInput, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct BuilderInput {
    ident: Ident,
    data: Data<(), BuilderField>,
}
```

This says:

- “Only accept named structs.”
- “Look for `#[builder(...)]` helper attributes.”
- “While you’re at it, parse every field using `BuilderField` (next section).”

### Field-level parsing with `FromField`

```rust
use darling::FromField;
use syn::{Ident, Type};

#[derive(FromField)]
#[darling(attributes(builder))]
struct BuilderField {
    ident: Option<Ident>,
    ty: Type,

    #[darling(default)]
    each: Option<String>,
}
```

This says:

- “Give me the field’s name and type.”
- “If there’s a `#[builder(each = "...")]`, parse it into `each: Option<String>`.”
- “If there isn’t, don’t error — just use `None`.”

### Putting it together: the new `derive_builder`

Remember that 50-line `old_way_parse` function? We can delete it entirely.

Now the derive macro entry point becomes… pleasantly boring:

```rust
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match BuilderInput::from_derive_input(&input) {
        Ok(parsed) => derive_builder_impl(parsed).into(),
        Err(e) => e.write_errors().into(),
    }
}
```

**💡 Aha!** Notice `e.write_errors()`—darling generates helpful error messages automatically, including “Did you mean?” suggestions for typos!

And *now* your code generation step (`derive_builder_impl`) can focus on generating the builder, not decoding attribute syntax.

---

## Darling Attributes Cheat Sheet

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `#[darling(attributes(name))]` | Which helper attrs to parse | `attributes(builder)` → parses `#[builder(...)]` |
| `#[darling(default)]` | Optional with `Default` value | Field becomes `Option<T>` or uses `T::default()` |
| `#[darling(rename = "x")]` | Different attribute name | `rename = "type"` since `type` is a keyword |
| `#[darling(forward_attrs)]` | Preserve raw attributes | Keep `#[doc]` attrs intact |
| `#[darling(supports(...))]` | Restrict to struct/enum | `supports(struct_named)` = named structs only |
| `#[darling(multiple)]` | Collect repeated attrs | `#[tag("a")] #[tag("b")]` → `vec!["a", "b"]` |

### Defaults Example

```rust
#[derive(FromField)]
#[darling(attributes(field))]
struct MyField {
    ident: Option<Ident>,
    ty: Type,
    
    // Uses false if not specified
    #[darling(default)]
    skip: bool,
    
    // Uses None if not specified  
    #[darling(default)]
    rename: Option<String>,
    
    // Custom default
    #[darling(default = "default_format")]
    format: String,
}

fn default_format() -> String {
    "{:?}".to_string()
}
```

---

## Error Handling Magic

Darling's error messages are *chef's kiss*. If someone writes:

```rust
#[builder(eac = "arg")]  // Typo: 'eac' instead of 'each'
```

Darling generates:

```text
error: Unknown field: `eac`. Did you mean `each`?
  --> src/main.rs:5:14
   |
 5 |     #[builder(eac = "arg")]
   |              ^^^
```

No manual error handling required!

### Accumulating Multiple Errors

Darling can collect multiple errors instead of stopping at the first:

```rust
match BuilderInput::from_derive_input(&input) {
    Ok(parsed) => { /* use parsed */ }
    Err(e) => return e.write_errors().into(),
}
```

If there are 3 attribute errors, the user sees all 3 at once—much better UX than fixing one, recompiling, finding another...

---

## FromMeta: Parsing the Inside of `#[attr(...)]`

Sometimes you want to parse the *arguments* of an attribute as their own struct (especially when the attribute has multiple options).

For example, these:

```rust
#[builder(each = "arg", rename = "args", default)]
```

…are a mini “configuration language”. `FromMeta` lets you model that configuration directly:

```rust
use darling::FromMeta;

#[derive(FromMeta)]
struct BuilderArgs {
    #[darling(default)]
    each: Option<String>,

    #[darling(default)]
    rename: Option<String>,

    #[darling(default)]
    default: bool,
}
```

You won’t always need `FromMeta`, but it’s the secret weapon when you want your attribute syntax to scale without turning into spaghetti.

---

## When NOT to Use Darling

Darling is fantastic, but sometimes overkill:

1. **Simple single-value attributes**: `#[repr(C)]` doesn't need darling
2. **Maximum control**: When you need custom error messages or complex validation
3. **Minimal dependencies**: Darling adds compile time

For quick one-offs, manual parsing might be simpler:

```rust
// Simple enough to not need darling
fn has_skip_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("skip"))
}
```

---

## Real Example: Builder with Darling

The workshop’s real Builder macro in `builder/src/lib.rs` follows the exact pattern you just saw:

- Parse `TokenStream` → `syn::DeriveInput`
- Parse `DeriveInput` → `BuilderInput` (darling)
- Generate code in a separate `derive_builder_impl` function (quote)

If you want to see the “full, working” version (including `each`, `Option<T>` handling, and the generated setters), that file is the best reference.

---

## Key Takeaways

📌 **darling is "serde for attributes"** — Define structs, get parsing free.

📌 **`FromDeriveInput` for struct-level** — Ident, generics, struct-level attrs.

📌 **`FromField` for field-level** — Field ident, type, field-level attrs.

📌 **`#[darling(default)]` for optional** — No more `Option<Option<T>>` gymnastics.

📌 **Errors are automatic and helpful** — "Did you mean?" suggestions included.

📌 **`write_errors()` returns all errors** — Better UX than one-at-a-time.

---

## Try It Yourself

The example in [`examples/04_darling_attrs/`](./examples/04_darling_attrs/) shows before/after:

```bash
cd docs/proc_macro_tutorial/examples/04_darling_attrs
cargo run --example demo 2>&1
```

---

## Next Up

You've got `syn` for parsing, `quote` for generating, and `darling` for attributes. What about when you need to transform identifier names? `user_name` → `UserName` → `USER_NAME`?

That's `heck`—a tiny crate that makes case conversion trivial.

**[Continue to Chapter 5: Case Conversion with heck →](./05-heck.md)**

---

*[← Previous: Code Generation with quote](./03-quote.md)*

