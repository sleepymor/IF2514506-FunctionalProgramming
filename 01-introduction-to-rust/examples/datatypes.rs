fn main(){
    // 1. Type Inference (Defaults)
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string
   
    // 2. Explicit Type Annotation
    let my_num2: i32 = 10;          // integer
    let my_double2: f64 = 10.99;    // float
    let my_letter2: char = 'E';     // character
    let my_bool2: bool = false;     // boolean
    let my_text2: &str = "World";   // string
}