use guessing_game::init;
use log::info;
use std::io;

fn main() {
    init();
    info!("Guess the number!");
    info!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    info!("You guessed: {}", guess);


}
