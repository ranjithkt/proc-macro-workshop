//! Demo of darling attribute parsing.
//!
//! Run with: cargo run --example demo 2>&1

#![allow(dead_code)]

use darling_attrs::ConfigDerive;

// Example 1: Full config with all attribute types
#[derive(ConfigDerive, Default)]
#[config(prefix = "MYAPP_")]
struct AppConfig {
    /// Uses custom env var name
    #[config(env = "DATABASE_URL")]
    database_url: String,

    /// Uses prefix + field name in uppercase: MYAPP_HOST
    host: String,

    /// Has a default value
    #[config(default = "8080")]
    port: String,

    /// Skipped - uses Default::default()
    #[config(skip)]
    internal_state: String,
}

// Example 2: Minimal config without struct-level attributes
#[derive(ConfigDerive, Default)]
struct SimpleConfig {
    #[config(env = "API_KEY")]
    api_key: String,

    #[config(default = "https://api.example.com")]
    api_url: String,
}

fn main() {
    println!("Darling attribute parsing demo!");
    println!();
    println!("Check the stderr output above to see what darling parsed.");
    println!();
    println!("The beauty of darling:");
    println!("  1. Define a struct with the fields you want");
    println!("  2. Add #[derive(FromField)] or #[derive(FromDeriveInput)]");
    println!("  3. Darling generates all the parsing code!");
    println!();
    println!("Compare src/lib.rs (darling) vs src/manual.rs (no darling)");
    println!("to see the difference in code complexity.");
}

