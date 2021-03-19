use rand::Rng;
use std::io::{self, Write};

// main loop running game logic
fn main() {
    println!("Number guessing game");
    println!("--------------------");

    let secret = secret();
    let mut count: u8 = 1;

    loop {
        let guess = get_input(count);
        count += 1;

        // converts guess to u8
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match check(guess, secret, count) {
            true => break,
            false => continue,
        }
    }
}

// Generates the secret number
fn secret() -> u8 {
    let secret = rand::thread_rng().gen_range(1..101);
    secret
}

// Takes user string input
fn get_input(count: u8) -> String {
    
    let mut guess = String::new();

    print!("{}) Input your guess: ", count);

    io::stdout()
        .flush()
        .expect("Unable to read line.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line.");

    println!("Analyzing {}", guess);
    println!("--------------------");

    guess
}

// Checks if input guess is equal to secret
fn check(num: u8, secret: u8, count: u8) -> bool {
    let mut output: bool = false;

    match (num, secret) {
        (..) if num == secret => {
            output = true; 
            println!("Thats correct!");
            println!("It took you {} tries.", count - 1);
        },
        (..) if num > secret => {
            println!("Too big.");
            println!("--------------------");
        },
        _ => {
            println!("Too small.");
            println!("--------------------");
        }
    }
    
    output
}
