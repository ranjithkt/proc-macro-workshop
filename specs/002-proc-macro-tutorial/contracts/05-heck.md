# Contract: Chapter 05 - Case Conversion with heck

## Purpose
Introduce the heck crate for transforming identifier casing in generated code.

## Content Requirements

### What You'll Learn
- Why case conversion matters in macros
- heck's case conversion traits
- Common patterns for generated identifiers

### Sections

1. **The Problem: Naming Generated Methods** (~150 words)
   - Field `user_name` needs getter `get_user_name` or `getUserName`
   - Builder field `args` needs setter `arg` (singular)
   - Manual string manipulation is error-prone

2. **heck's Case Conversion Traits** (~300 words)
   - Table of traits:
   | Trait | Input | Output |
   |-------|-------|--------|
   | ToSnakeCase | "UserName" | "user_name" |
   | ToPascalCase | "user_name" | "UserName" |
   | ToCamelCase | "user_name" | "userName" |
   | ToKebabCase | "UserName" | "user-name" |
   | ToShoutySnakeCase | "userName" | "USER_NAME" |
   | ToTitleCase | "user_name" | "User Name" |

3. **Basic Usage** (~200 words)
   ```rust
   use heck::ToSnakeCase;
   
   let field_name = "UserEmail";
   let getter = format!("get_{}", field_name.to_snake_case());
   // getter = "get_user_email"
   ```

4. **Pattern: Builder Setters** (~200 words)
   - Field name to setter method
   - Example from workshop's builder macro
   ```rust
   use heck::ToPascalCase;
   
   let field = "email_address";
   let builder = format!("{}Builder", struct_name);
   let setter = format_ident!("set_{}", field);
   ```

5. **Pattern: Enum Variant Serialization** (~150 words)
   - Variant names to string representations
   - ToKebabCase for CLI options
   - ToShoutySnakeCase for environment variables

6. **Combining with format_ident!** (~150 words)
   ```rust
   use heck::ToSnakeCase;
   use quote::format_ident;
   
   let method_name = format_ident!(
       "get_{}",
       field.ident.to_string().to_snake_case()
   );
   ```

7. **Key Takeaways**
   - heck provides simple case conversion
   - Use traits: ToSnakeCase, ToPascalCase, etc.
   - Combine with format_ident! for clean generated names

### Diagrams Required
None (utility-focused chapter)

## Estimated Time
10 minutes

## Dependencies
Chapter 03 (quote) - format_ident! usage

## Acceptance Criteria
- Reader can use heck for case conversion
- Reader knows common patterns
- Reader can combine with format_ident!

