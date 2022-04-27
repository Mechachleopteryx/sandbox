use std::io;

fn main() {
    println!("Guess what number I am thinking of.");

    println!("What is your guess?");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("I'm sorry Dave, I'm afraid I can't do that right now");

    println!("You guessed: {}", guess);
}
