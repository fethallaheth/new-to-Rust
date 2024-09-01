// Importing the external crate for language detection
use whatlang::{detect};
use std::io::{self, Write};

// Function to detect and display language information
fn what_language(text: &str) {
    // Use `unwrap()` for simplicity in handling potential errors
    // In a real application, you might want to handle errors more gracefully
    match detect(text) {
        Some(info) => {
            println!("Language: {}", info.lang());
            println!("Script: {}", info.script());
            println!("Confidence: {}", info.confidence());
            println!("Reliable: {}", info.is_reliable());
        },
        None => {
            println!("Language detection failed.");
        }
    }
}

fn main() {
    // Prompt the user to enter a dialog in any language
    print!("Enter a dialog in any language: ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Create a mutable string to store the user input
    let mut input = String::new();

    // Read the user input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove any leading or trailing whitespace from the input
    let trimmed_input = input.trim();

    // Call the function to detect the language of the trimmed input
    what_language(trimmed_input);
}


// I think the more words you write the more accurate the result will be.

// Arabic :: مرحبا بالجميع في هدا العالم 
// korean :: 안녕 녹
// English :: Hello rust


// 한국어   == korean 
// العربية == Arabic