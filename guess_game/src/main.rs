use std::io;
use std::cmp::Ordering;
use rand::RngExt;

fn main() {
    println!("This is a guess game!!!");
    //Generate a secret number to guess
    let secret_number: u8 = rand::rng().random_range(0..=100);

    loop {
        println!("Guess a number: ");
        //Create a variable
        let mut guess: String = String::new();

        //Input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to input");

        //Convert a string to a integer

        //let guess:u8 = guess.trim().parse().expect("Failed to convert"); --> Break when input not a number
        
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Your guess greater than secret number"),
            Ordering::Less => println!("Your guess less than secret number"),
            Ordering::Equal => {
                println!("YOU WIN!!!");
                break;
            }
        }
    }
}
