//! Demo of token debugging macros.
//!
//! Run with: cargo run --example demo 2>&1
//! The 2>&1 captures stderr where eprintln! writes.

#![allow(dead_code)]

use token_debug::{count_tokens, debug_tokens, inspect_tokens};

// Try uncommenting different examples to see the token output!

// Example 1: A simple struct
debug_tokens!(
    struct Foo {
        x: i32,
        y: String,
    }
);

// Example 2: Inspect a function signature (with body so it's valid Rust)
inspect_tokens!(
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
);

// Example 3: Count tokens in an impl block
trait MyTrait {
    fn method(&self) -> bool;
}
struct MyStruct;

count_tokens!(
    impl MyTrait for MyStruct {
        fn method(&self) -> bool {
            true
        }
    }
);

fn main() {
    println!("Token debug examples completed!");
    println!("Check the stderr output above to see the token analysis.");
    println!();
    println!("Try modifying this file and re-running to explore how");
    println!("different Rust constructs become tokens.");
}
