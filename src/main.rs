extern crate rand;
extern crate getopts;

use std::io;
use std::env;
use std::path::Path;
use std::cmp::Ordering;
use rand::Rng;
use getopts::Options;

/** Prompts the user for an input, and returns the result as
 * an integer. If an invalid entry is given, read_integer will
 * loop until a valid one is received. */
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

/** Plays the guessing game. Asks the user to enter an upper
 * bound, and picks a secret number between 1 and UPPER_BOUND.
 * Instructs the user to guess the number. If 'wizard' is true,
 * the answer is printed at the beginning of the game. */
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

/** Prints the usage for this program. */
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

/** Returns the basename of the executable, as determined by
 * the contents of argv[0]. */
fn get_progname()->String {
    let args:Vec<String> = env::args().collect();
    let program_name = Path::new(&args[0]).file_name().unwrap();
    return program_name.to_str().unwrap().to_string();
}

/** Main program. Creates and executes an option parser, and then
 * launches the guessing-game. */
fn main() {
    let program_name = get_progname();
    let args:Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let mut wizard = false;

    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("w", "wizard", "Print the answer, you cheater");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => {panic!(f.to_string())}
    };

    if matches.opt_present("h") {
        print_usage(&program_name, opts);
        return;
    }
    
    if matches.opt_present("w") {
        wizard = true;
    }
    
    guessing_game(wizard);
}
