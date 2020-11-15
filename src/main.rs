use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("Welcome to Guess A Number!");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin()");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);
        println!("The secret number was: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low."),
            Ordering::Equal => {
                println!("You guessed Correctly!");
                break;
            }
            Ordering::Greater => println!("You guessed too high"),
        }
    }
}
