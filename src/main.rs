use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};
fn main() {
    println!("This is a guessing game!");
    println!("Please enter your guess (Numbers are between 1 and 10):");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number was {secret_number}")
}
