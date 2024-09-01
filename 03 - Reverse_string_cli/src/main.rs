use std::io::{self, Write};

fn main () {
    // prompt the user for input 
    println!("Enter a String to Reverse : ");
    io::stdout().flush().unwrap();

    // Read the input from the user 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the newline character from the input 
    input = input.trim().to_string();

    // Reverse the input string 
    let reversed = reverse_string(&input);

    // Output the reversed string

    println!("Reversed String : {}", reversed);
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
