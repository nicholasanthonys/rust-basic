fn main() {
    // this code will produce error to prevent freeing memory twice.'
    // read the concept : https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    // Ways Variables and Data Interact: Clone
    // This works just fine and explicitly produces the behavior shown in Figure 4-3, where the heap data does get copied.
    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 {}", s1, s2);

    // Stack-Only Data: Copy`
    // But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Ownership and Functions

    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x);

    // this code will produce error because variable s has been moved`
    // let s2 = s ;
    // println!("s2 : {s2}");

    // Return Values and Scope
    // Returning values can also transfer ownership. Listing 4-4 shows an example of a function that returns some value

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("s2 : {s3}");

    // Transferring ownership of return values
    //The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.
    // What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

    // Return Multipel value
    // Rust does let us return multiple values using a tuple,
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // The issue with the tuple code in Listing 4-5 is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length. Instead, we can provide a reference to the String value. A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

    // References and Borrowing
    // When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.
    // We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
    let s1 = String::from("hello");
    let len = calculate_length_reference(&s1);
    println!("The length of '{}', is {}.", s1, len);

    // Mutable References
    // We can fix the code from before  to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:

    let mut s = String::from("hello");
    change(&mut s);
    println!("after change, s is : {s}");

    // RESTRICTION of mutable references
    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
    // Uncomment this code will produce error
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data

    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
    // Uncomment this code will produce error
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);
    // Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
    //Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed. The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)

    // Dangling References
    //In languages with pointers, it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
    //Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:

    let reference_no_nothing = dangle();

    // The slice type
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

    // String Slices
    //A string slice is a reference to part of a String, and it looks like this:
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So these are equal:
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    // uncomment This code will produce error
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    // / dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

//  Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.
// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

//  When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
