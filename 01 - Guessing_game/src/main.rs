use std::io;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess the number between 1 and 100: ");
        let mut user_input: String = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read input");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue;
            }
        };

        if user_input < secret_number {
            println!("Too small!");
        } else if user_input > secret_number {
            println!("Too big!");
        } else {
            println!("You guessed it! It's {}!", secret_number);
            break;
        }
    }
}