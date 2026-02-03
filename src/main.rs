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
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_string: &str = "     my string      ";
    println!("{}", empty_string.trim());

    println!("{}", value.pow(5));
}