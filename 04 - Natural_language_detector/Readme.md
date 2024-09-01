

## Project-04 - Language Detection CLI

### Overview
This command-line tool detects the language of a given text input. Utilizing the `whatlang` crate, the program analyzes the text provided by the user and returns information about the detected language, including the language name, script, confidence level, and reliability of the detection.

### Code Explanation

#### Imports

```rust
use whatlang::{detect};
use std::io::{self, Write};
```
- `whatlang`: This external crate is used for detecting the language of the input text.
- `std::io`: Provides functionality for input and output operations, including reading from and writing to the console.

#### Function Definition

```rust
fn what_language(text: &str) {
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
```
- **Purpose**: The `what_language` function takes a string slice (`&str`) as input, detects its language, and prints the details.
- **Language Detection**: The `detect` function from the `whatlang` crate returns an `Option` containing language information if successful.
- **Error Handling**: If language detection fails (`None`), the function prints a failure message.

#### Main Function

```rust
fn main() {
    print!("Enter a dialog in any language: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();

    what_language(trimmed_input);
}
```
- **User Prompt**: The program prompts the user to enter text in any language. The `print!` macro is used to avoid an extra newline before user input.
- **Reading Input**: The program reads the user's input from the console into a mutable `String` variable.
- **Trimming Input**: The `trim()` method is used to remove any leading or trailing whitespace from the input.
- **Language Detection**: The trimmed input is passed to the `what_language` function for language detection.

### Usage
1. **Run the Program**: Execute the compiled binary to start the program.
2. **Enter Text**: Type or paste the text you want to analyze and press Enter.
3. **View Results**: The program will output the detected language, script, confidence level, and reliability of the detection.

### Example
```
Enter a dialog in any language: مرحبا بالجميع في هدا العالم
Language: Arabic
Script: Arab
Confidence: 0.99
Reliable: true
```

### Notes

- **Accuracy**: The detection accuracy generally improves with the length of the input text. Longer texts provide more context for better detection results.



