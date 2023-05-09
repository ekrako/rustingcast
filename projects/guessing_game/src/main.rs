use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret);
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}