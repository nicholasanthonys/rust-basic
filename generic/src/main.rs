fn main() {
    // Removing Duplication by Extracting a Function
    // find largest number

    let number_list = vec![34, 50, 25, 100, 65];
    let mut result = &number_list[0];

    for number in &number_list {
        if number > result {
            result = number;
        }
    }

    // println!("The largest number is {}", largest);

    // extract to a function

    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // Generic Data Types

    // Won't work.
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_generic(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_generic(&char_list);
    // println!("The largest char is {}", result);

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.1, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("p.x = {}", both_float.x());

    enum Option<T> {
        Some(T),
        None,
    }

    // We can also specify constraints on generic types when defining methods on the type
    // impl Point<f32> {
    //     fn distance_from_origin(&self) -> f32 {
    //         (self.x.powi(2) + self.y.powi(2)).sqrt()
    //     }
    // }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    println!(
        "p3.x = {}, p3.y = {}",
        integer_and_float.x, integer_and_float.y
    );

    // Traits: Defining Shared Behavior
    use generic::{NewsArticle, Summary, Tweet};

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Traits as Parameters

    // Instead of a concrete type for the item parameter,
    // we specify the impl keyword and the trait name.
    // This parameter accepts any type that implements the specified trait
    // This is a syntax sugar for code below
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound Syntax
    // The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:
    // pub fn notify<T: Summary>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    notify(&article);

    // Specifying Multiple Trait Bounds with the + Syntax
    // We can also specify more than one trait bound. Say we wanted notify to use display formatting as well as summarize on item: we specify in the notify definition that item must implement both Display and Summary. We can do so using the + syntax:
    // pub fn notify(item: &(impl Summary + Display)) {

    // The + syntax is also valid with trait bounds on generic types:
    // pub fn notify<T: Summary + Display>(item: &T) {

    // Clearer Trait Bounds with where Clauses
    // INstead of
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // we can use where clause:
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    // }

    // Returning Types that Implement Traits
    // The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // Using Trait Bounds to Conditionally Implement Methods
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // Preventing Dangling References with Lifetimes
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    // Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // won't compile
    // The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y. Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y!
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // Lifetime Annotations in Function Signatures
    // To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.
    // We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the lifetime 'a and then add it to each reference, as shown in
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Let’s look at how the lifetime annotations restrict the longest function by passing in references
    // Using the longest function with references to String values that have different concrete lifetimes
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // Next, let’s try an example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments
    // Won't compile
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    //we can look at this code and see that string1 is longer than string2 and therefore result will contain a reference to string1. Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the println! statement. However, the compiler can’t see that the reference is valid in this case. We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. Therefore, the borrow checker disallows the code in Listing 10-23 as possibly having an invalid reference.

    // Thinking in Terms of Lifetimes
    // The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter
    // fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    //     x
    // }

    // When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters
    // won't compile
    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
    // Here, even though we’ve specified a lifetime parameter 'a for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.

    // Lifetime Annotations in Struct Definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // Lifetime Elision
    // You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references. However, in Chapter 4 we had a function in Listing 4-9, shown again in Listing 10-25, that compiled without lifetime annotations.
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. At that time, the function signature would have been written like this:
    // fn first_word<'a>(s: &'a str) -> &'a str {

    /// Lifetime Annotations in Method Definitions
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // The lifetime parameter declaration after impl and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime
    //One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:

    let s: &'static str = "I have a static lifetime.";

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    //Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

// this is a function that return largest number
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// this i sa function that return largest_char
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Won't work
// fn largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
