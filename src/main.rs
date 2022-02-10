extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number =  rand::thread_rng().gen_range(1,101);

    // println!("The secret number is: {}", secret_number);    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        // let foo = 5; // immutable
        // let mut bar = 5; // mutable
        io::stdin().read_line(&mut guess)
            .expect("faild to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
