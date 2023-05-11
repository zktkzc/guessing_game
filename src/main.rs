use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    if cfg!(debug_assertions) {
        println!("The secret_number is: {secret_number}.");
    } else {
        println!("The secret_number is: {}", "***");
    }

    loop {
        println!("Please input your guess(1-100).");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
