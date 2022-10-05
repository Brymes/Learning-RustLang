use std::in
// Rng means random number generator
use rand::Rng;

fn main() {
    println!("Hello! Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Input your guess here:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // --read line of input, and store it in the variable specified--
        // --read_line also returns a Result value--
        .expect("Failed to read line, or input not set.");  //handling potential failure
        // --without the expect, the program will compile but yield warning that you haven't used--
        // --the Result value from read_line--
    println!("You guessed: {guess}");
}