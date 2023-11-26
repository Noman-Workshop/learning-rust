use std::cmp::Ordering;
use std::io;
use std::io::{stdout, Write};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        let mut guess = String::new();
        // Here new is an associated function
        // associated functions is a function that is implemented on the type

        print!("Please input your guess: ");
        stdout().flush().expect("");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a valid number", guess.trim());
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!")
        }
    }
}