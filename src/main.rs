use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Enter guess trial number");
    let mut guess_trials = String::new();
    io::stdin()
        .read_line(&mut guess_trials)
        .expect("Expect to pass the number of guess");
    let guess_trials: u8 = guess_trials
        .trim()
        .parse()
        .expect("Expect guess amount to be of a numeric type");

    let mut actuals = Vec::new();

    for _ in 0..guess_trials {
        actuals.push(rand::thread_rng().gen_range(1..=10));
    }

    println!("Guess a number");

    let mut guess_count = 0;

    while guess_count < guess_trials {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Expect a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid numeric value, try again!");
                continue;
            }
        };

        match guess.cmp(&actuals[guess_count as usize]) {
            Ordering::Greater => {
                println!("You guess too high")
            }
            Ordering::Less => {
                println!("You guess too low")
            }
            Ordering::Equal => {
                println!("You guessed right");
                guess_count += 1;
            }
        };
    }
}
