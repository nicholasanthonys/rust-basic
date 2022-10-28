use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;
fn main() {
    // try_panic();

    // panic_backtrace();

    // Recoverable Errors with Result

    //Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.
    // Recall from “Handling Potential Failure with the Result Type” in Chapter 2 that the Result enum is defined as having two variants, Ok and Err, as follows:

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    //In the case where File::open succeeds, the value in the variable greeting_file_result will be an instance of Ok that contains a file handle. In the case where it fails, the value in greeting_file_result will be an instance of Err that contains more information about the kind of error that happened.
    // if you remove file hello.txt, this will produce error
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Alternatives to Using match with Result<T, E>
    let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The Result<T, E> type has many helper methods defined on it to do various, more specific tasks. The unwrap method is a shortcut method implemented just like the match expression we wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:
    let greeting_file = File::open("hello.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of expect looks like this:
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Cases in Which You Have More Information Than the Compiler

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

fn try_panic() {
    panic!("crash and burn");
}

// Let’s look at another example to see what it’s like when a panic! call comes from a library because of a bug in our code instead of from our code calling the macro directly.
// RUN : RUST_BACKTRACE =1 cargo run
fn panic_backtrace() {
    // Let’s look at another example to see what it’s like when a panic! call comes from a library because of a bug in our code instead of from our code calling the macro directly.
    let v = vec![1, 2, 3];

    v[99];
}

// Propagating Errors
// When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
// The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code
fn read_username_from_file_question() -> Result<String, io::Error> {
    // The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler.

    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)
    fs::read_to_string("hello.txt")

    // Where The ? Operator Can Be Used
    // The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression we defined in Listing 9-6. In Listing 9-6, the match was using a Result value, and the early return arm returned an Err(e) value. The return type of the function has to be a Result so that it’s compatible with this return.
}

// The error message also mentioned that ? can be used with Option<T> values as well. As with using ? on Result, you can only use ? on Option in a function that returns an Option
// The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point. If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
fn last_char_of_first_line(text: &str) -> Option<char> {
    // next() will return option because it’s possible that the first line is the empty string
    // Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.
    // The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.

    text.lines().next()?.chars().last()
}

// Luckily, main can also return a Result<(), E>. Listing 9-12 has the code from Listing 9-10 but we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile:
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// Creating Custom Types for Validation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
