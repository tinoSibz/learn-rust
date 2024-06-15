use colored::*;
use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("{}","Guess the number between 1 and 100!".blue().bold());

    // Generate secret number between 1 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number has been generated: {secret_number}");

    // Keep track of attempts
    let mut attempts = 0;

    loop {
        attempts += 1;
        // Read user guess
        let mut guess = String::new();
        println!("Please enter your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read line");

        // Convert guess string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Your guess was not a number!".red().bold());
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!!!".red()),
            Ordering::Greater => println!("{}", "Too big!!!".red()),
            Ordering::Equal => {
                println!("{} {attempts} {}", "You win!".green().bold(), "needed");
                break;
            }
        }
    }
}
