use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let actual = rand::thread_rng().gen_range(1..=10);
    println!("Guess a number");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Expect a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid numeric value, try again!");
                continue;
            }
        };

        match guess.cmp(&actual) {
            Ordering::Greater => {
                println!("You guess too high")
            }
            Ordering::Less => {
                println!("You guess too low")
            }
            Ordering::Equal => {
                println!("You guessed right");
                break;
            }
        };
    }
}
