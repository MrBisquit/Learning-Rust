use std::{cmp::Ordering, io};
use rand::Rng;
mod maths;
mod auto_guess;

fn main() {
    do_some_maths();

    // Ask them the max number (Default: 100)
    println!("Input the max (Default: 100): ");
    let mut max = String::new();
    io::stdin().read_line(&mut max).expect("Failed to read line");
    let max: i32 = match max.trim().parse() {
        Ok(num) => num,
        Err(_) => 100
    };

    let secret_number = rand::thread_rng().gen_range(1..=max);

    println!("Guess the number");
    /*println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }*/

    // Making it more compact
    println!("First, let the computer do a round.");
    let mut last: i32 = auto_guess::Guess(0, auto_guess::Input::None, max);
    let mut rounds: i32 = 0;
    loop {
        rounds += 1;

        let mut input: auto_guess::Input = auto_guess::Input::None;

        match last.cmp(&secret_number) {
            Ordering::Less => input = auto_guess::Input::Higher,
            Ordering::Greater => input = auto_guess::Input::Lower,
            Ordering::Equal => {
                break;
            }
        }

        last = auto_guess::Guess(last, input, max);
    }
    println!("It guessed correctly ({last}) in {rounds} rounds!");

    loop {
        println!("Your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}

fn do_some_maths() {
    let added: i32 = maths::add(10, 10);
    println!("10 + 10 = {added}");
    let takenaway: i32 = maths::takeaway(10, 5);
    println!("10 - 5  = {takenaway}");
    let multiplied: i32 = maths::multiply(10, 10);
    println!("10 * 10 = {multiplied}");
    let divided: i32 = maths::divide(10, 5);
    println!("10 / 5  = {divided}");
    let powered: i32 = maths::power(10, 5);
    println!("10 ^ 5  = {powered}");
    let squarerooted: i32 = maths::square_root(10);
    println!("Sqrt 10 = {squarerooted}");

    maths::square_root(999999999);
}