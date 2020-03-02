use rand::Rng;
use std::cmp::Ordering;
use std::io;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
fn main() {
    println!("Guess the number:");

    let secret_number = rand::thread_rng().gen_range(1, 21);

    loop {
        println!("Please input yoour guess.");

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
