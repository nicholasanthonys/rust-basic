mod front_of_house;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
}

// Consider the code in Listing 7-8 that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function fix_incorrect_order defined in the back_of_house module calls the function deliver_order defined in the parent module by specifying the path to deliver_order starting with super
// The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root. From there, we look for deliver_order and find it. Success! We think the back_of_house module and the deliver_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant_public() {
    // Making Structs and Enums Public
    // We can also use pub to designate structs and enums as public, but there are a few details extra to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. In Listing 7-9, we’ve defined a public back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field. This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal based on what’s in season and in stock. The available fruit changes quickly, so customers can’t choose the fruit or even see which fruit they’ll get.

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_enum() {
    // In contrast, if we make an enum public, all of its variants are then public
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public.
// Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub

// Creating Idiomatic use Paths
pub fn use_keyword() {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting::add_to_waitlist;
    use std::collections::HashMap;

    pub fn eat_at_restaurant() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        add_to_waitlist();
    }
    // Providing New Names with the as Keyword
    use std::fmt::Result;
    use std::io::Result as IoResult;

    // fn function1() -> Result {
    //     // --snip--
    // }

    // fn function2() -> IoResult<()> {
    //     // --snip--
    // }

    // Re-exporting Names with pub use
    // When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use

    pub use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    // Using External Packages
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Note that the standard std library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std. But we do need to refer to it with use to bring items from there into our package’s scope.
    // use std::collections::HashMap;

    // Using Nested Paths to Clean Up Large use Lists
    // use std::cmp::Ordering;
    //use std::io;
    use std::{cmp::Ordering, io};

    // We can use a nested path at any level in a path,
    //which is useful when combining two use statements that share a subpath
    // use std::io;
    // use std::io::Write;
    // into
    // use std::io::{self, Write};

    // The Glob Operator
    use std::collections::*;
    // This use statement brings all public items defined in std::collections into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined
}
