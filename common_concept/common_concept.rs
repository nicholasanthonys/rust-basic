// constant
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    println!("constant : {THREE_HOURS_IN_SECONDS} ");

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // isize or usize based on 64 or 32 bit architecture
    let a: isize = 32;
    println!("variable a: {a}");

    let mut literal = 98_222;
    literal += 1;
    println!("literal : {literal}");

    // to handle integer overlfow: : https://doc.rust-lang.org/book/ch03-02-data-types.html

    // Wrap in all modes with the wrapping_* methods, such as wrapping_add
    // Return the None value if there is overflow with the checked_* methods
    // Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    // Saturate at the value’s minimum or maximum values with saturating_* methods

    // numeric operation
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    // boolean type
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // character type
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y} {y} {z}");

    //The Array Type
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    // Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack
    // an array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector
    // However, arrays are more useful when you know the number of elements will not need to chang
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let first = a[0];
    println!("{}", first);

    // function
    another_function(20);
    print_labeled_measurement(10, 'h');

    // expression vs statement
    // statement doesn't return
    // expression have a return val : calling a function, calling a macro, a new scope block

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Functions with Return Values
    // Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->)
    // There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i3
    fn five() -> i32 {
        // expression
        5
    }
    println!("{}", five());

    // should be 6
    let x = plus_one(5);

    println!("The value of x is: {x}");
    fn plus_one(x: i32) -> i32 {
        x + 1
        // add semicolon, so this line bellow become a statement. This will produce error
        // x + 1;
    }

    // control flow
    // if expressions

    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Copndition was false");
    }

    // the condition must be a bool. uncomment this line to produce error.
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling Multiple Conditions with else if
    let number = 6;

    // Using too many else if expressions can clutter your code, so if you have more than one, you might want to
    // refactor your code. Chapter 6 describes a powerful Rust branching construct called match for these cases.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable, as in Listing 3-2.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");

    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type; in Listing 3-2, the results of both the if arm and the else arm were i32 integers
    // uncomment this code will produce error
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // Repetition with Loops
    // Rust has three kinds of loops: loop, while, and for. Let’s try each one.

    // Repeating Code with loop
    //The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    // this will produce infinie loop

    // loop {
    //     println!("again")
    // }

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    // If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:

    // The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first break that doesn’t specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop.
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    // You can choose to use the while construct to loop over the elements of a collection, such as an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // However, this approach is error prone; we could cause the program to panic if the index value or test condition are incorrect. For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

    // As a more concise alternative, you can use a for loop and execute some code for each item in a collection

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {element}")
    }

    // reverse number
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // for loop index
    let list: &[i32] = &vec![1, 3, 4, 17, 81];

    for (i, el) in list.iter().enumerate() {
        println!("The current element is {}", el);
        println!("The current index is {}", i);
    }
}

// Note that we defined another_function after the main function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller
fn another_function(x: i32) {
    println!("Value of x : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
