extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // Generate a random number in range of 1 to 100 (1 - 100)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    // Code executed while the user don't won
    loop {
        println!("Enter your guess.");
        
        // Create a mutable string variable for store the user input
        let mut guess = String::new();
        
        // Get the user input and store it in "guess" variable
        io::stdin().read_line(&mut guess)
            .expect("Error on read the guess");
        
        // Convert the "guess" variable from String to u32 (numeric)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You said {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too hight!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}
