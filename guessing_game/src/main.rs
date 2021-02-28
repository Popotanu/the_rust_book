use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number tanu-!");
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("the secret number is: {}", secret_number);

    println!("lets input your guess, tanu.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {} tanu!", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too big!"),
        Ordering::Equal => println!("tanu----!!!"),
    }

}
