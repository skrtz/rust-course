fn main() {

    /*
    String Literals in Rust
    ---------------------------
    In Rust, string literals are sequences of characters enclosed in double quotes.
    They are immutable and have a fixed size known at compile time.
    String literals can also include escape sequences for special characters. 
    */
    let normal_str = "This is a string literal.";
    let raw_str = r#"This is a raw string literal with a "quote" inside."#;

    println!("{}", normal_str);
    println!("{}", raw_str);

    /*
    Escape Sequences
    ---------------------------
    Rust supports several escape sequences in string literals:  
    - \n: Newline
    - \t: Tab
    - \\: Backslash
    - \": Double quote
    - \r: Carriage return
    - \0: Null character
    */
}