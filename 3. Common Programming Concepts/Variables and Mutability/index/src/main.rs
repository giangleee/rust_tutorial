fn main () {
    // VARIABLE
    /* variable x is immutable => like const, can not change */
    let x = 5;
    /* if x is mutable => can change value */
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    // CONSTANT
    /* -    Can not use "mut" keyword with constant, because it's alaway
    mutable.
       -    Have to defind type of const (u32, i32, u64, i64)*/
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("{THREE_HOURS_IN_SECONDS}");
    //SHADOWING
    /* Can use variable as a result of a function  */
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /* In mut case: have to use same type of variable, when unset mut dont have to do that */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");
}
