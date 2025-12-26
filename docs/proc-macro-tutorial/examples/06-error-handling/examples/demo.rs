//! Demo of error handling with proc-macro-error2.
//!
//! Run with: cargo run --example demo 2>&1

use error_handling::Validated;

// This struct should compile fine - all validation passes
#[derive(Validated)]
struct GoodStruct {
    id: u64,
    name: String,
    other: i32,
}

// Uncomment these to see error handling in action:

// This would trigger the "id must be u64" error:
// #[derive(Validated)]
// struct BadId {
//     id: String,  // Error: should be u64
//     name: String,
// }

// This would trigger the "name must be String" error:
// #[derive(Validated)]
// struct BadName {
//     id: u64,
//     name: i32,  // Error: should be String
// }

// This would trigger MULTIPLE errors at once:
// #[derive(Validated)]
// struct MultipleErrors {
//     id: i32,     // Error: should be u64
//     name: bool,  // Error: should be String
// }

// This would trigger a fatal error:
// #[derive(Validated)]
// struct TupleStruct(u64, String);  // Error: tuple structs not supported

fn main() {
    println!("Error handling demo!");
    println!();
    println!("The GoodStruct compiled successfully because it passed validation.");
    println!();
    println!("To see error handling in action, uncomment the commented-out");
    println!("structs above and try to compile. You'll see:");
    println!();
    println!("  1. BadId - single error about id type");
    println!("  2. BadName - single error about name type");
    println!("  3. MultipleErrors - TWO errors shown at once!");
    println!("  4. TupleStruct - fatal error with help text");
    println!();
    println!("The difference between emit_error! and abort!:");
    println!("  - emit_error! accumulates errors, continues processing");
    println!("  - abort! stops immediately (for fatal errors)");

    let good = GoodStruct {
        id: 42,
        name: "test".to_string(),
        other: 100,
    };
    println!();
    println!("GoodStruct validates: {}", good.validate());
}

