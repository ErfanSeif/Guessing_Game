use std::io;
use rand::Rng;

fn main() {
    println!("Welcome To The Guessing Game!");
    //---    

    let randNumber = rand::thread_rng().gen_range(1..=100);
    println!("Random Number Is: {}",randNumber);
    //---    

    let mut guess = String::new();

    println!("What's Your Guess?  ");

    std::io::stdin()
        .read_line( &mut guess)
        .expect("Failed To Read Line...");

    println!("You Guessed: {}" , guess);
    //---    

}