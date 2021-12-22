use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing number game");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        println!("Please input your guess");
        // because read_line returns Result with values of Err or Ok
        // we use expect method to crash the program if Err returned
        io::stdin().read_line(&mut guess).expect("Error");

        // parse returns Result as well and also parse needs type specified.
        // we trim string because every time
        // user enters a string it is actually not "vale" but "value\n"
        let guess = match guess.trim().parse::<u32>() {
            // match will prevent our program from crashing
            Ok(num) => num, //return number of Result is Ok
            Err(_) => {
                // _ is a catch all value
                // and print hint and continue asking for a number
                println!("Please enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break; // this is neat feature tha we could declare a block of code like that
            }
        }
    }
}
