# Rust Learning Projects

Welcome to my Rust learning journey! This repository contains a series of mini-projects that I have completed while learning the Rust programming language. Each project focuses on different aspects of Rust, from basic syntax and control flow to more advanced concepts like memory management and concurrency.

## Projects 

- [**Project-01 - Guessing Game**](https://github.com/fethallaheth/rust-projects/tree/master/guessing_game)
  
  #### Overview
  This is a simple command-line guessing game where the user has to guess a randomly generated number between 1 and 100. The game provides feedback on whether the guessed number is too small, too big, or correct. The game continues until the correct number is guessed.

  #### What I Learned
  - Basic Rust syntax, including `fn` for functions, `let` for variable declaration, and the `loop` keyword.
  - How to use the `rand` crate to generate random numbers.
  - How to handle user input with the `std::io` module.
  - Basic error handling in Rust using `expect`.
  - Parsing strings to integers using `trim()` and `parse()`.

### [Project-02 - Simple Calculator](https://github.com/fethallaheth/rust-projects/tree/master/simple_calculator)

#### Overview
This is a simple command-line calculator that allows the user to input two numbers and perform basic arithmetic operations (addition, subtraction, multiplication, and division). The program handles input parsing, validates the user's input, and performs the selected operation.

#### What I Learned
- How to use the `std::io` module to handle input and output in Rust.
- Implementing basic arithmetic operations with functions.
- Using `f64::is_nan` to validate input.
- Understanding the `match` statement for control flow.
- Parsing and converting string inputs to floating-point numbers.