use rand::Rng; // for random number generation
use std::cmp::Ordering; // for comparing, Ordering between less, greater, and equal.
use std::io; // for obtaining input/output from user to console

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

    let guess: i32 = guess.trim()
        .parse()
        .expect("please enter a number!");

    println!("You guessed: {guess}!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too big!"),
        Ordering::Equal => println!("You win!"),
    }
}