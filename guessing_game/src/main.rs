use std::io;
use rand::Rng;
use std::cmp::Ordering;

extern crate rand;

fn main() {
    println!("Guess the number between 1-100");
    let _secret_number = rand::thread_rng().gen_range(1, 101);
    let mut chances_left = 3;
    while chances_left > 0 {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        chances_left -= 1;
        if chances_left != 0 {
            println!("You've {} chance(s) left\n", chances_left);
        }
    }
}
