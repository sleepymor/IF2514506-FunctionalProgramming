// Example: functions in Rust
// Run with: cargo run --example functions

// A simple function that adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon = this value is returned
    // or we could write `return a + b;` with a semicolon
}

// A function that returns nothing
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // 1. Call a function and save the result
    let result = add(5, 10);
    println!("5 + 10 = {}", result);

    // 2. Call a function that just prints something
    greet("John");
    greet("Alice");
}
