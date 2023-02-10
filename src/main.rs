// To obtain user input and print the result as output - called the prelude
use std::io;

// For less than, greater than, equal operations
use std::cmp::Ordering;

// To generate a random number
use rand::Rng;

// The main function
fn main() {
    // Prints a string "Guess the number!"
    println!("Guess the number!");

    // Variable to hold random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Temp: Show secret number
    //println!("Secret number: {secret_number}");

    // Add a loop to keep the user guessing without ending the program
    loop {
        // Print a string to input the guess
        println!("Please input your guess.");

        // Store the user input - "mut" allows it to be mutatable; basically like "const"
        let mut guess = String::new();

        // Receive the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim() = remove whitespace
        // parse() = convert from string to another type
        // u32 = tells rust the type is a 32-bit 
        // parse () only works on characters that can logically be converted into numbers; A👍% would not convert to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Replaced: "let guess: u32 = guess.trim().parse().expect("Please type a number!");"
                        

        println!("You guessed: {guess}");

        // Compare - use match to determine what to do next based on which variant of Ordering was returned from the call to cmp with the value in guess and secret_number
        // match expresson is made up of arms. An arm consists of a pattern to match against
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break out of game
            }
        }
    }
}
