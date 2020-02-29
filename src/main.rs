use std::io;

fn main() {
    println!("Guess the number:");
    
    println!("Please input yoour guess.");
    
    let mut guess = String::new(); // mutable
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed: {}", guess);
}
