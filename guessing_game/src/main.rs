extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let secret = rand::thread_rng().gen_range(1,101);
   
}