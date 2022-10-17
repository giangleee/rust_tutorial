// io: input/output library into scope
// io library comes from a standart library call std
use std::io;
use  rand::Rng;

fn main() {
    println!("Guess the Number");
    println!("Please input your guess!");

    // another definition of "Let" statement
    let apple = 5;
    // apple variable CAN NOT change once it is defind
    // if you want to change the value, use "mut" statement
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);


    println!("The secret number is: {secret_number}");

    io::stdin() // use library
        .read_line(&mut guess)// address of variable, insted of using &guess => &mut guess to let that variable mutable
        .expect("Failed to read line"); // catch exception
    
    println!("You guessed: {guess}");
    // or
    println!("You guessed: {}", guess);
}
