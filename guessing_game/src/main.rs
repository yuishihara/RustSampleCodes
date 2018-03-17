extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number");

    let secret_low = 1;
    let secret_high = 101;
    let secret_number = rand::thread_rng().gen_range(secret_low, secret_high);

    println!("The secret number is {}", secret_number);

    guessing_game(&secret_number);
}

fn guessing_game(secret_number: &u32) {
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed reading number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(secret_number) {
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
