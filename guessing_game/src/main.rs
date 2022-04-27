use rand::Rng;
use std::io;
use std::cmp::Ordering; // on by default

fn main() {
    println!("Guess what number I am thinking of.");

    let arcanum = rand::thread_rng().gen_range(1..101);


    //  println!("What is your guess?");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("I'm sorry Dave, I'm afraid I can't do that right now");

    let guess: u32 = guess.trim().parse().expect("What's your gues?");

    println!("You guessed: {}", guess);

    match guess.cmp(&arcanum) {
        Ordering::Less => println!("Too little."),
        Ordering::Greater => println!("Too big."),
        Ordering::Equal => println!("You chose wisely!")
    }
}
