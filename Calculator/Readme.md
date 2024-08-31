
#### Overview
This project is a simple command-line calculator application written in Rust. It allows users to input two numbers and select an arithmetic operation to perform on those numbers. The calculator supports addition, subtraction, multiplication, and division. The program demonstrates basic concepts in Rust, including input handling, type conversion, and basic arithmetic operations.

#### Detailed Code Explanation

1. **Imports and Main Function**
   ```rust
   use std::io;

   fn main() {
       // Main code logic here
   }
   ```
   - **`use std::io;`**: This imports the `std::io` module, which provides functionality for handling input and output operations.
   - **`fn main() { ... }`**: This is the main function, the entry point of the program. It contains the primary logic for the calculator.

2. **Input Parsing Function**

   ```rust
   fn input_parser() -> f64 {
       let mut x: String = String::new();
       io::stdin().read_line(&mut x).expect("Invalid Input");
       let x: f64 = match x.trim().parse() {
           Ok(num) => num,
           Err(_) => {
               return f64::NAN;
           }
       };

       return x;
   }
   ```
   - **`fn input_parser() -> f64 { ... }`**: Defines a function `input_parser` that reads a line of input from the user and attempts to convert it to a `f64`.
   - **`let mut x: String = String::new();`**: Creates a mutable `String` variable to store the user input.
   - **`io::stdin().read_line(&mut x).expect("Invalid Input");`**: Reads a line of input from the standard input and stores it in `x`. If reading fails, the program will panic with "Invalid Input".
   - **`let x: f64 = match x.trim().parse() { ... }`**: Trims any whitespace from the input and attempts to parse it as a `f64`. If parsing fails, it returns `f64::NAN` (Not a Number) to indicate an invalid input.

3. **Main Logic**

   ```rust
   fn main() {
       let result: f64;

       println!("Enter the first number: ");
       let x: f64 = input_parser();

       if f64::is_nan(x) {
           println!("Invalid input!");
           return;
       }

       println!("Enter the second number: ");
       let y: f64 = input_parser();

       if f64::is_nan(y) {
           println!("Invalid input!");
           return;
       }

       println!("List of operators:");
       println!("(1) Add");
       println!("(2) Subtract");
       println!("(3) Multiply");
       println!("(4) Divide");
       println!("Select the number associated with the desired operation: ");

       let op: f64 = input_parser();

       if f64::is_nan(op) {
           println!("Invalid input!");
           return;
       }

       let op: i32 = op as i32;

       match op {
           1 => result = add(x, y),
           2 => result = subtract(x, y),
           3 => result = multiply(x, y),
           4 => result = divide(x, y),
           _ => {
               println!("Invalid selection");
               return;
           }
       }

       println!("The result is: {}", result);
   }
   ```
   - **`let result: f64;`**: Declares a variable `result` to store the result of the selected arithmetic operation.
   - **`println!("Enter the first number: ");`**: Prompts the user to enter the first number.
   - **`let x: f64 = input_parser();`**: Calls `input_parser` to get the first number from the user.
   - **`if f64::is_nan(x) { ... }`**: Checks if the input is valid (not `NaN`). If not, prints an error message and exits the program.
   - **`println!("Enter the second number: ");`**: Prompts the user to enter the second number.
   - **`let y: f64 = input_parser();`**: Calls `input_parser` to get the second number from the user.
   - **`println!("List of operators: ...");`**: Displays a list of available arithmetic operations.
   - **`let op: f64 = input_parser();`**: Calls `input_parser` to get the user's choice of operation.
   - **`let op: i32 = op as i32;`**: Converts the operation choice from `f64` to `i32` for use in the `match` statement.
   - **`match op { ... }`**: Executes the appropriate arithmetic function based on the user's choice:
     - **`1 => result = add(x, y),`**: Calls the `add` function if the user chooses addition.
     - **`2 => result = subtract(x, y),`**: Calls the `subtract` function if the user chooses subtraction.
     - **`3 => result = multiply(x, y),`**: Calls the `multiply` function if the user chooses multiplication.
     - **`4 => result = divide(x, y),`**: Calls the `divide` function if the user chooses division.
     - **`_ => { ... }`**: Handles invalid selections by printing an error message and exiting.

4. **Arithmetic Functions**

   ```rust
   fn add(x: f64, y: f64) -> f64 {
       return x + y;
   }

   fn subtract(x: f64, y: f64) -> f64 {
       return x - y;
   }

   fn multiply(x: f64, y: f64) -> f64 {
       return x * y;
   }

   fn divide(x: f64, y: f64) -> f64 {
       return x / y;
   }
   ```
   - **`fn add(x: f64, y: f64) -> f64 { ... }`**: Defines a function that returns the sum of two numbers.
   - **`fn subtract(x: f64, y: f64) -> f64 { ... }`**: Defines a function that returns the difference between two numbers.
   - **`fn multiply(x: f64, y: f64) -> f64 { ... }`**: Defines a function that returns the product of two numbers.
   - **`fn divide(x: f64, y: f64) -> f64 { ... }`**: Defines a function that returns the quotient of two numbers.

---
