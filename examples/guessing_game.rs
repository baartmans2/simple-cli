//To run this example `cargo run --example guessing_game --release`

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let the_number = rand::thread_rng().gen_range(1..=100);
    simple_cli::clear_terminal();
    let mut number_of_guesses = 0;
    loop {
        let input = simple_cli::get_number::<i8>(
            Some("Pick a number between 1 and 100!"),
            Some("Try Again."),
            Some(1),
            Some(100),
        );
        simple_cli::clear_terminal();
        number_of_guesses += 1;
        match input.cmp(&the_number) {
            Ordering::Less => println!("{} is too low!", input),
            Ordering::Greater => println!("{} is too high!", input),
            Ordering::Equal => {
                println!(
                    "YOU WIN!\n{} was the secret number!\nNumber of guesses: {}",
                    input, number_of_guesses
                );
                break;
            }
        }
    }
}
