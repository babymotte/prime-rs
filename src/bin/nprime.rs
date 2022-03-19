use std::{env, process::exit};

use prime_rs::is_prime;

fn main() {
    match env::args().skip(1).next() {
        Some(number) => {
            if let Ok(number) = number.parse() {
                match (number..u64::MAX).find(|n| is_prime(*n)) {
                    Some(n) => println!("{}", n),
                    None => {
                        eprintln!("no prime found");
                        exit(1);
                    }
                }
            } else {
                eprintln!("Not a positive integer.");
                exit(2);
            }
        }
        None => {
            eprintln!("No number specified.");
            exit(1);
        }
    }
}
