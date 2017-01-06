extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn read_integer(prompt:&str)->u32 {
    let mut input = String::new();

    loop {
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse() {
            Err(_) => continue,
            Ok(num) => return num,
        }
    }
}

fn main() {
    println!("Guess the number!");
    
    let bound = read_integer("Set the upper bound.");
    let secret_number = rand::thread_rng().gen_range(1,bound + 1);
    
    //println!("The secret number is: {}", secret_number);
    
    loop {
        let guess = read_integer("Please input your guess.");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
