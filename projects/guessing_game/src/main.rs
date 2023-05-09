use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret);
    println!("Guess the number!");
    let mut guess = String::new();
    loop {
        println!("Please input your guess.");


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess_int: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                return;
            }
        };
        match guess_int.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        };
    };
}