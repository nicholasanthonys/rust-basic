use rand::Rng;
use std::cmp::Ordering;
/**
 * To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std
 */
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // print secret number
    // println!("The scret number is: {secret_number}");

    loop {
        println!("Please input your guess");
        //   let apples = 5; // immutable
        //   let mut bananas = 5; // mutable

        // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind
        // mutable variable
        let mut guess = String::new();

        //. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // elimiinate space using trim, \n, \r, \r\n for example when user press enter, it adds \n to the string
        // We create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // continue the loop if error e.g:input is string
        // Remember that parse returns a Result type and Result is an enum that has the variants Ok and Err. We’re using a match expression here, as we did with the Ordering result of the cmp metho
        //  The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // print more than one value using curly bracket
        // let x = 5;
        // let y = 10;

        // println!("x = {} and y = {}", x, y);

        // comparing
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
