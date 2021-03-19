use rand::Rng;
use std::cmp::Ordering;
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

        if check(guess, secret, count) == true {
            break;
        } else {
            continue;
        }
    }
}

// Generates the secret number
fn secret() -> u8 {
    rand::thread_rng().gen_range(1..101)
}

// Takes user string input
fn get_input(count: u8) -> u8 {
    
    let mut guess = String::new();
    let mut out = io::stdout();

    print!("{}) Input your guess: ", count);
    out.flush().expect("Unable to flush stdout.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line.");

    println!("Analyzing {}", guess);
    println!("--------------------");

    let guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Numbers only... lol.");
            get_input(count+1)
        }
    };

    guess
}

// Checks if input guess is equal to secret
fn check(num: u8, secret: u8, count: u8) -> bool {
    let mut output: bool = false;

    match num.cmp(&secret) {
        Ordering::Equal => {
            output = true;
            println!("Thats correct!");
            println!("It took you {} tries.", count - 1);
        },
        Ordering::Greater => {
            println!("Too big.");
            println!("--------------------");
        },
        Ordering::Less => {
            println!("Too small.");
            println!("--------------------");
        }
    }
    
    output
}
