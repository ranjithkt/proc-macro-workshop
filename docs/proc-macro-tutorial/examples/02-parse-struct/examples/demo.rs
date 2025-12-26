//! Demo of struct parsing macros.
//!
//! Run with: cargo run --example demo 2>&1

use parse_struct::{DebugParse, ListFields};

// Example 1: A simple struct with named fields
#[derive(DebugParse)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

// Example 2: A tuple struct
#[derive(DebugParse)]
struct Point(f64, f64, f64);

// Example 3: A unit struct
#[derive(DebugParse)]
struct Marker;

// Example 4: A struct with generics
#[derive(DebugParse)]
struct Container<'a, T: Clone> {
    data: &'a T,
    count: usize,
}

// Example 5: An enum
#[derive(DebugParse)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// Example 6: Struct with attributes on fields
#[derive(ListFields)]
struct Config {
    #[my_attr]
    debug_mode: bool,

    #[my_attr = "special"]
    log_level: String,

    // No attribute
    port: u16,
}

fn main() {
    println!("Struct parsing examples completed!");
    println!("Check the stderr output above to see the parsed structures.");
    println!();
    println!("Notice how syn extracts:");
    println!("  - Struct/enum names");
    println!("  - Field names and types");
    println!("  - Generics and lifetimes");
    println!("  - Attributes on fields");
}

