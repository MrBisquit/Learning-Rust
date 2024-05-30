use std::{cmp::Ordering, io};
use rand::Rng;
mod maths;

fn main() {
    do_some_maths();

    let secret_number = rand::thread_rng().gen_range(1..=100);

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
    loop {
        println!("Your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
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
}