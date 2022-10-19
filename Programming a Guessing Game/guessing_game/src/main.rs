// io: input/output library into scope
// io library comes from a standart library call std
use rand::Rng;
use core::num;
use std::io;
// Ordering is a enum library to compare two number or string
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop { // Loop unti meet exception
        println!("Please input your guess!");

        // another definition of "Let" statement
        let _apple = 5;
        // apple variable CAN NOT change once it is defind
        // if you want to change the value, use "mut" statement
        let mut guess = String::new();

        io::stdin() // use library
            .read_line(&mut guess) // address of variable, insted of using &guess => &mut guess to let that variable mutable
            .expect("Failed to read line"); // catch exception

        // Rust allows us to shadow the previous value of guess with a new one
        /* Shadowing lets us reuse the guess variable name rather than forcing
        us to create two unique variables, such as guess_str and guess for example. */
        // first "guess": guess type int
        // second "guess": guess type string
        // ":": chú thích thay đổi loại biến với kiểu u32

        // let guess: u32 = guess
        //     .trim()
        //     .parse() // turn string => another type
        //     .expect("Please type a number!"); // catch exception
        
        // or

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");
        // or
        println!("You guessed: {}", guess);

        // like Switch
        // cmp stand for compare
        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small"),
        //     Ordering::Greater => println!("Too Big"),
        //     Ordering::Equal => println!("You win"),
        // }
        //or
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                break;
            },
            Ordering::Greater => {
                println!("Too Big");
                break;
            },
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
