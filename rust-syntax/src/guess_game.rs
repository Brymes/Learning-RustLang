use std::in

fn main() {
    println!("Hello! Guess a number!");

    println!("Input your guess here:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // read line of input, and store it in the variable specified
        .expect("Failed to read line, or input not set.");  //handling potential failure

    println!("You guessed: {guess}");
}