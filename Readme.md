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
### [Project-04 - Natural Language Detector](https://github.com/fethallaheth/rust-projects/tree/master/04%20-%20Natural_language_detector)

#### Overview
This project is a command-line tool that detects the language of the input text using the whatlang crate. The program reads a string provided by the user, analyzes it to determine the language, script, confidence level, and reliability of the detection, and then outputs these details to the console.

#### What I Learned
- Language Detection: Utilized the whatlang crate to identify the language of the input text.
- User Input Handling: Managed user input and output using the std::io module, including reading from the console and flushing output.
- String Processing: Worked with string slices (&str) and the String type for text manipulation.
- Error Handling: Implemented basic error handling for cases where language detection may fail, using match for more robust error handling.
- CLI Development: Built a simple command-line interface (CLI) application that interacts with users and processes their input.

--- 
### [Project-05 - Task_manager](https://github.com/fethallaheth/rust-projects/tree/master/05%20-%20Task_manager)

#### Overview
The Task Manager is a simple command-line interface (CLI) tool written in Rust that helps users manage their to-do list. The tool allows users to add tasks, list all tasks, and mark tasks as completed. It is a beginner-friendly project designed to demonstrate basic concepts in Rust, including struct usage, method implementation, and control flow.

#### Key Features:
- **Add Tasks**: Users can add new tasks to their to-do list.
- **List Tasks**: The tool displays all tasks along with their completion status.
- **Complete Tasks**: Users can mark tasks as completed.

#### What I Learned:
- **Structs and Methods**: How to define and implement methods for custom data types.
- **Vector Manipulation**: Storing and manipulating a dynamic list of tasks.
- **Input Handling**: Reading and processing user input from the command line.
- **Control Flow**: Using loops and pattern matching to create an interactive menu.
- **Error Handling**: Managing potential errors and ensuring a smooth user experience.