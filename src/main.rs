use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN_RANGE: u32 = 1;
const MAX_RANGE: u32 = 21;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
fn main() {
    println!("Guess the number:");

    let secret_number = rand::thread_rng().gen_range(MIN_RANGE, MAX_RANGE);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
