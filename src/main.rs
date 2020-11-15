use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //generate random number and write a welcome message to screen
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("Welcome to Guess A Number!");


    //loop until break is called
    loop {
        println!("Please input your guess: ");

        //get input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin()");

        //trim and cast user input to an int32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease input a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        // println!("The secret number was: {}", secret_number);

        //compare inputed number to the random number, match is similar to switch
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low."),
            Ordering::Equal => {
                println!("You guessed Correctly!");
                break; //break out of the loop because the number ws guessed
            }
            Ordering::Greater => println!("You guessed too high"),
        }
    }
}
