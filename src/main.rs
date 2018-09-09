extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop { 
        println!("🔢  Input your guess!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("❌  Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("💩  Wrong value provided. Try again!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("🔼  Too small!"),
            Ordering::Greater => println!("🔽  Too big!"),
            Ordering::Equal => {
                println!("🔑  You won!");
                break;
            }
        }
    }
}
