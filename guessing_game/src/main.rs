use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        println!("Please input your guess:");

        //println!("The secret number is: {}", secret_number);

        stdin().read_line(&mut guess).expect("Fail to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            }
        }
    }
}
