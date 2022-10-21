use std::io;

fn main() {
    // if we dont give more infomation about guess variable => error
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");
    // FLOATING POINT TYPES:
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    //Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    
    //BOOLEAN TYPE
    let t = true;

    let f: bool = false; // with explicit type annotation

    //The Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //Compound Types
    /* The Tuple Type */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // or
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    // or
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    /* The Array Type */
    let a = [1, 2, 3, 4, 5];
    // or
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // or
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    let a = [3; 5];


    // TEMP PROGRAM
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
