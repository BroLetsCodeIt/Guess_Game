// importing the io library
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to Guess the Number Game! [stop -> ctrl+c]");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The Secret number is : {secret_number}");

    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small 🤪"),
            Ordering::Greater => println!("Too Big 😯"),
            Ordering::Equal => {
                println!("🌟 You Win!");
                break;
            }
        }
    }
}
