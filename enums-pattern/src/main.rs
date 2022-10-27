use std::net::IpAddr;

enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    // Enums allow you to define a type by enumerating its possible variants. First, we’ll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum. Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.

    // Defining an Enum

    // Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.
    // Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate, because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

    // Enum Values
    //We can create instances of each of the two variants of IpAddrKind like this:

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind
    // And we can call this function with either variant:
    route(four);
    route(six);

    // Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is. Given that you just learned about structs in Chapter 5, you might be tempted to tackle this problem with structs as shown in Listing 6-1.
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
    // his new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:

    // enum IpAddr {
    //     V4(String),
    //     v6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // We attach data to each variant of the enum directly, so there is no need for an extra struct.
    // There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.

    // Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // However, as it turns out, wanting to store IP addresses and encode which kind they are is so common that the standard library has a definition we can use! Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:

    struct Ipv4Addr {
        // -- snip --
    }

    struct Ipv6Addr {
        // --snip--
    }

    // This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum!
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // Note that even though the standard library contains a definition for IpAddr, we can still create and use our own definition without conflict because we haven’t brought the standard library’s definition into our scope. We’ll talk more about bringing types into scope in Chapter 7.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // This enum has four variants with different types:
    // Quit has no data associated with it at all.
    // Move has named fields like a struct does.
    // Write includes a single String.
    // ChangeColor includes three i32 values.

    // Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions, except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type. The following structs could hold the same data that the preceding enum variants hold:
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
                                              // But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum

    // There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write((String::from("hello")));
    m.call();

    // The Option Enum and Its Advantages Over Null Values

    // This section explores a case study of Option, which is another enum defined by the standard library. The Option type encodes the very common scenario in which a value could be something or it could be nothing.
    //For example, if you request the first of a list containing items, you would get a value. If you request the first item of an empty list, you would get nothing.
    // Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.

    // Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.
    // As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the standard library as follows:

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    //  Its variants are also included in the prelude: you can use Some and None directly without the Option::

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    // The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type. Rust can infer these types because we’ve specified a value inside the Some variant. For absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>
    // When we have a Some value, we know that a value is present and the value is held within the Some. When we have a None value, in some sense, it means the same thing as null: we don’t have a valid value. So why is having Option<T> any better than having null?

    // In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>:
    // this code will produce error

    // let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x+y;

    //Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.

    // When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value.
    // We can proceed confidently without having to check for null before using that value. Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

    //  In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null.
    // o, how do you get the T value out of a Some variant when you have a value of type Option<T> so you can use that value? Th
    // num has a large number of methods that are useful in a variety of situations; you can check them out in its documentation. Becoming familiar with the methods on Option<T> will be extremely useful in your journey with Rust.

    // In general, in order to use an Option<T> value, you want to have code that will handle each variant. You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T. You want some other code to run if you have a None value, and that code doesn’t have a T value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

    // The match Control Flow Construct
    // Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things; Chapter 18 covers all the different kinds of patterns and what they do. The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.
    // Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.
    // Speaking of coins, let’s use them as an example using match! We can write a function that takes an unknown United States coin and, in a similar way as the counting machine, determines which coin it is and return its value in cents, as shown here in Listing 6-3.

    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }

    // fn value_in_cents(coin: Coin) -> u8 {
    //     match Coin {
    //         Coin::Penny => {
    //             println!("Lucky Penny");
    //             1
    //         }
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    // Let’s break down the match in the value_in_cents function. First, we list the match keyword followed by an expression, which in this case is the value coin.
    //  This seems very similar to an expression used with if, but there’s a big difference:
    // with if, the expression needs to return a Boolean value, but here, it can return any type. The type of coin in this example is the Coin enum that we defined on the first line

    // Patterns that Bind to Values
    // Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    // Let’s imagine that a friend is trying to collect all 50 state quarters. While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so if it’s one our friend doesn’t have, they can add it to their collection.
    fn value_in_cents_state(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    // value_in_cents_state(Coin::Quarter(UsState::Alaska));
    value_in_cents_state(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    // Does Some(5) match Some(i)? Why yes it does! We have the same variant. The i binds to the value contained in Some, so i takes the value 5. The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

    // Matches Are Exhaustive
    // There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities. Consider this version of our plus_one function, which has a bug and won’t compile:
    // uncomment this code will produce error
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // Catch-all Patterns and the _ Placeholder
    // Using enums, we can also take special actions for a few particular values, but for all other values take one default action. Imagine we’re implementing a game where, if you roll a 3 on a dice roll, your player doesn’t move, but instead gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all other values, your player moves that number of spaces on the game board. Here’s a match that implements that logic, with the result of the dice roll hardcoded rather than a random value, and all other logic represented by functions without bodies because actually implementing them is out of scope for this example:

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed. This catch-all pattern meets the requirement that match must be exhaustive. Note that we have to put the catch-all arm last because the patterns are evaluated in order. If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!
    // Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: _

    // is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // Finally, we’ll change the rules of the game one more time, so that nothing else happens on your turn if you roll anything other than a 3 or a 7
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Concise Control Flow with if let

    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // Instead, we could write this in a shorter way using if let. The following code behaves the same as the match above
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    // We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else. Recall the Coin enum definition in Listing 6-4, where the Quarter variant also held a UsState value.
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Or we could use an if let and else expression like this:
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {}
