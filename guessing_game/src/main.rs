use colored::*;
use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("{}","Guess the Secret Number (between 1 and 100)".blue().bold());
    let game_mode = choose_game_mode();

    // Generate secret number between 1 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("{}","The secret number has been generated".blue());

    // Run the game for the specified game mode
    match game_mode {
        1 => human_player(secret_number),
        2 => computer_player(secret_number),
        _ => println!("{}","Invalid game mode selected.".red().bold()),
    }
}

fn choose_game_mode() -> u32 {
    println!("{}", "Pick Game Mode".blue().bold());
    println!("1. Human Player (default)");
    println!("2. Computer Player (binary search algorithm)");

    let mut game_mode = String::new();
    io::stdin()
        .read_line(&mut game_mode)
        .expect("Error: Failed to read line");

    let game_mode : u32 =  game_mode.trim().parse().unwrap_or_else(|_| 1);
    return game_mode
}

fn human_player(secret_number: u32) {
    // Keep track of attempts
    let mut attempts = 0;
    loop {
        attempts += 1;
        // Read user guess
        let mut guess = String::new();
        println!("\nPlease enter your guess");
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
                println!("{} {attempts} {}", "You win!".green().bold(), "attempts needed");
                break;
            }
        }
    }
}

fn binary_search(low: u32, high: u32) -> u32 {
    return (low + high)/2
}

fn computer_player(secret_number: u32) {
    // Keep track of attempts
    let mut attempts = 0;
    let mut low: u32 = 1;
    let mut high: u32 =100;
    loop {
        attempts += 1;

        // Convert guess string to u32
        let mut guess: u32 = binary_search(low, high);
        println!("Computer guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => {
                println!("{}", "Too small!!!".red());
                low = guess;
            },
            Ordering::Greater => {
                println!("{}", "Too big!!!".red());
                high = guess;
            },
            Ordering::Equal => {
                println!("{} {attempts} {}", "Computer wins!".green().bold(), "attempts needed");
                break;
            }
        }
    }
}