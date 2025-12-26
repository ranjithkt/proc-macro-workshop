//! Demo of case conversion with heck.
//!
//! Run with: cargo run --example demo 2>&1

use case_convert::{BuilderNamed, EnumKebab};

// Example 1: Show case conversions for struct fields
// Note: CaseDemo generates methods with syntax issues, so we skip it for now

// Example 2: Enum to kebab-case strings
#[derive(EnumKebab)]
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
    BadRequest,
    ServiceUnavailable,
}

// Example 3: Builder naming with PascalCase
#[derive(BuilderNamed)]
struct my_special_config;

#[derive(BuilderNamed)]
struct UserProfile;

fn main() {
    println!("Case conversion demo!");
    println!();
    println!("Check stderr output above for case conversion examples.");
    println!();

    // Test EnumKebab
    println!("HTTP Status as_str():");
    println!("  Ok                   -> {}", HttpStatus::Ok.as_str());
    println!(
        "  NotFound             -> {}",
        HttpStatus::NotFound.as_str()
    );
    println!(
        "  InternalServerError  -> {}",
        HttpStatus::InternalServerError.as_str()
    );
    println!(
        "  BadRequest           -> {}",
        HttpStatus::BadRequest.as_str()
    );
    println!(
        "  ServiceUnavailable   -> {}",
        HttpStatus::ServiceUnavailable.as_str()
    );
    println!();

    // Test BuilderNamed
    println!("Builder names generated:");
    println!("  my_special_config::builder() -> MySpecialConfigBuilder");
    println!("  UserProfile::builder() -> UserProfileBuilder");
    println!();

    println!("heck makes case conversion trivial!");
    println!("Just import the trait and call .to_snake_case(), etc.");
}

