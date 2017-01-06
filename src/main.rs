extern crate rand;
extern crate getopts;

use std::io;
use std::env;
use std::cmp::Ordering;
use rand::Rng;
use getopts::Options;

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

fn guessing_game(wizard:bool) {
    println!("Guess the number!");
    
    let bound = read_integer("Set the upper bound.");
    let secret_number = rand::thread_rng().gen_range(1,bound + 1);
    
    if wizard {
        println!("The secret number is: {}", secret_number);
    }
    
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
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    let mut wizard = false;

    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("w", "wizard", "Print the answer, you cheater");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => {panic!(f.to_string())}
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    if matches.opt_present("w") {
        wizard = true;
    }
    
    guessing_game(wizard);
}
