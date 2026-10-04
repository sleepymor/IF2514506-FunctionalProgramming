// Example: data types in Rust
// Run with: cargo run --example datatypes

fn main() {
    // 1. Rust can guess the type (type inference)
    let my_num = 5; // integer
    let my_double = 5.99; // float
    let my_letter = 'D'; // character
    let my_bool = true; // boolean
    let my_text = "Hello"; // string slice (&str)
    println!(
        "{} {} {} {} {}",
        my_num, my_double, my_letter, my_bool, my_text
    );

    // 2. Or we can write the type ourselves
    let my_num2: i32 = 10;
    let my_double2: f64 = 10.99;
    let my_letter2: char = 'E';
    let my_bool2: bool = false;
    let my_text2: &str = "World";
    println!(
        "{} {} {} {} {}",
        my_num2, my_double2, my_letter2, my_bool2, my_text2
    );

    // 3. Tuple: a group of mixed values
    let person = ("John", 30, true);
    println!(
        "Name: {}, Age: {}, Is student: {}",
        person.0, person.1, person.2
    );

    // 4. Array: a list of same-type values
    let numbers = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
    println!("All numbers: {:?}", numbers);
}
