use std::hash::Hash;

fn main() {
    // Creating a New Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors
    // There are two ways to reference a value stored in a vector: via indexing or using the get method.
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    //  this will produce error
    // let does_not_exist = &v[100];

    // this will return None without panicking
    let does_not_exist = v.get(100);
    // When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. Your code will then have logic to handle having either Some(&element) or None, as discussed in Chapter 6. For example, the index could be coming from a person entering a number. If they accidentally enter a number that’s too large and the program gets a None value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value. That would be more user-friendly than crashing the program due to a typo!

    // this will produce error
    // The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {}", first);

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator. We’ll talk more about the dereference operator in the “Following the Pointer to the Value with the Dereference Operator” section of Chapter 15.

        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    // Vectors can only store values that are the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!
    // For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum.
    // Then we can create a vector to hold that enum and so, ultimately, holds different types.

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
      //When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

    // String

    // Create new string
    // This line creates a new empty string called s, which we can then load data into
    let mut s = String::new();

    let data = "initial contents";

    //  Using the to_string method to create a String from a string literal
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    //  Using the String::from function to create a String from a string literal
    let s = String::from("initial contents");

    // Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s : {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing into Strings
    let s1 = String::from("hello");

    // Rust strings don’t support indexing.
    // This will produce error
    // let h = s1[0];

    // A final reason Rust doesn’t allow us to index into a String to
    // get a character is that indexing operations are expected to always
    // take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

    // Slicing Strings
    //Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:
    let hello = "Здравствуйте";
    let s = &hello[0..6];
    println!("s : {}", s);

    // If we were to try to slice only part of a character’s bytes with
    // something like &hello[0..1], Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

    // Methods for Iterating Over Strings

    for c in "Зд".chars() {
        println!("{}", c);
    }

    // Alternatively, the bytes method returns each raw byte, which might be appropriate for your domai
    for b in "Зд".bytes() {
        println!("{}", b);
    }

    // hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.insert(String::from("Yellow"), 50);

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score : {}", score);

    // Hash Maps and Ownership
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // uncomment This will produce error
    // We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.
    // let color = map.get(&field_name);

    let color = map.get(&(String::from("Favorite color")));

    println!("Color : {:?}", color);

    // Overwriting a Value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text. The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // By default, HashMap uses a hashing function called SipHash 
    //that can provide resistance to Denial of Service (DoS) attacks involving hash tables1. 
    // This is not the fastest hashing algorithm available, but the trade-off for better security 
    // that comes with the drop in performance is worth it. 
    //If you profile your code and find that the default hash function is too slow for your purposes, 
    //you can switch to another function by specifying a different hasher. 
    // A hasher is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

    println!("{:?}", map);
}
