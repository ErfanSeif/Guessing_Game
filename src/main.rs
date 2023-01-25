fn main() {
    println!("Welcome To The Guessing Game!");
    println!("What's Your Guess?  ");
    
    let mut guess = String::new();
    std::io::stdin()
        .read_line( &mut guess)
        .expect("Failed To Read Line...");

    println!("You Guessed: {}" , guess);


}
