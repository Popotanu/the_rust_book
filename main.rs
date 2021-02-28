use std::io;

fn main() {
    println!("guess the number tanu-!");
    println!("lets input your guess, tanu.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        // .expect("failed to read line");

    println!("you guessed: {} tanu!", guess)
}
