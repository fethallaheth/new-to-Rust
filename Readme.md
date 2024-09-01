# Rust Learning Projects

Welcome to my Rust learning journey! This repository contains a series of mini-projects that I have completed while learning the Rust programming language. Each project focuses on different aspects of Rust, from basic syntax and control flow to more advanced concepts like memory management and concurrency.

## Projects 

### [Project-01 - Guessing Game](https://github.com/fethallaheth/rust-projects/tree/master/01%20-%20Guessing_game)
  
  #### Overview
  This is a simple command-line guessing game where the user has to guess a randomly generated number between 1 and 100. The game provides feedback on whether the guessed number is too small, too big, or correct. The game continues until the correct number is guessed.

  #### What I Learned
  - Basic Rust syntax, including `fn` for functions, `let` for variable declaration, and the `loop` keyword.
  - How to use the `rand` crate to generate random numbers.
  - How to handle user input with the `std::io` module.
  - Basic error handling in Rust using `expect`.
  - Parsing strings to integers using `trim()` and `parse()`.

### [Project-02 - Simple Calculator](https://github.com/fethallaheth/rust-projects/tree/master/02%20-%20Calculator)

#### Overview
This is a simple command-line calculator that allows the user to input two numbers and perform basic arithmetic operations (addition, subtraction, multiplication, and division). The program handles input parsing, validates the user's input, and performs the selected operation.

#### What I Learned
- How to use the `std::io` module to handle input and output in Rust.
- Implementing basic arithmetic operations with functions.
- Using `f64::is_nan` to validate input.
- Understanding the `match` statement for control flow.
- Parsing and converting string inputs to floating-point numbers.

Here's a README section tailored for your string reversal project:

---

### [Project-03 - Reverse String CLI](https://github.com/fethallaheth/rust-projects/tree/master/03%20-%20Reverse_string_cli)

#### Overview
This project is a command-line tool that allows the user to input a string and returns the string in reverse order. The program demonstrates basic string manipulation in Rust, including reading user input, processing strings, and outputting results to the console.

#### What I Learned
- Handling user input using the `std::io` module.
- String manipulation in Rust, including trimming whitespace and reversing characters.
- Working with functions, including passing arguments and returning values.
- Understanding string slices (`&str`) and heap-allocated `String` types.
- Basics of Rust's memory management and ownership model.

---

