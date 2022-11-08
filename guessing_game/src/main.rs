use std::io; // for obtaining input/output from user to console
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100); // start..=end

    println!("the secret number is: {secret_number}");

    println!("Please input your guess:");
    let mut guess = String::new(); // String is a growable UTF-8 encoded bit of text

    io::stdin()
        .read_line(&mut guess) // read_line collects user input and assigns to a variable
                                                        // & signifies that the argument is a reference
        .expect("Failed to read line");

    println!("You guessed: {guess}!");
}
