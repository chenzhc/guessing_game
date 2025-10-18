use guessing_game::init;
use log::info;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    init();
    info!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..101);

    info!("The secret number is: {}", secret_number);
    
    loop {
        info!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        info!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => info!("Too small"),
            Ordering::Greater => info!("Too big!"),
            Ordering::Equal => { 
                info!("You win!");
                break;
            },
        }

    }

}
