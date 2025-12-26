//! Demo of code generation macros.
//!
//! Run with: cargo run --example demo 2>&1
//! Or see generated code: cargo +nightly expand --example demo

#![allow(dead_code)]

use generate_impl::{Getters, SimpleBuilder, SimpleDebug};

// Example 1: Custom Debug implementation
#[derive(SimpleDebug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// Example 2: Tuple struct Debug
#[derive(SimpleDebug)]
struct Point(f64, f64);

// Example 3: Getter methods
#[derive(Getters)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

// Example 4: Builder pattern
#[derive(SimpleBuilder)]
struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
}

fn main() {
    // Test SimpleDebug
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    println!("User debug: {:?}", user);

    let point = Point(3.0, 4.0);
    println!("Point debug: {:?}", point);

    // Test Getters
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
    };
    println!(
        "Config: host={}, port={}, debug={}",
        config.get_host(),
        config.get_port(),
        config.get_debug()
    );

    // Test Builder
    let cmd = Command::builder()
        .executable("cargo".to_string())
        .args(vec!["build".to_string(), "--release".to_string()])
        .env(vec!["RUST_LOG=debug".to_string()])
        .build()
        .unwrap();

    println!("Command: {} {:?}", cmd.executable, cmd.args);
}

