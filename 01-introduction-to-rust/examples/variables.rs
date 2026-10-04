// Example: variables in Rust
// Run with: cargo run --example variables

fn main() {
    // 1. Variables are immutable by default (cannot change)
    let name = "John";
    let age = 30;
    println!("My name is {} and I am {} years old.", name, age);

    // 2. Use `mut` to make a variable changeable
    let mut counter = 0;
    println!("Counter: {}", counter);
    counter += 1;
    println!("Counter: {}", counter);

    // 3. Constants (always uppercase, always need a type)
    const MAX_SCORE: u32 = 100;
    println!("Max score is {}", MAX_SCORE);

    // 4. Shadowing: reuse the same name with `let` again
    let x = 5;
    let x = x + 1; // new x shadows the old one
    println!("x is now {}", x);
}
