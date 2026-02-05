/*
Intro to Methods
---------------------------------------
A method is a function that lives on a value.
It's an action we can ask the value to execute.
Example:    
value.method()
---------------------------------------
*/
fn main() {
    // Examples of methods in Rust
    // Calling methods on integer values
    let value: i32 = -15;
    println!("{}", value.abs());

    // Trim whitespace from a string
    let empty_string: &str = "     my string      ";
    println!("{}", empty_string.trim());

    // Pass a parameter to a method
    println!("{}", value.pow(5));

    println!("{:?}", empty_string.find("my"));
}