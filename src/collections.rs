/*
Rust’s standard library includes a number
of very useful data structures called collections.
Most other data types represent one
specific value, but collections can contain multiple
values.
*/

// 3 Collections that are very often
/*
• A vector allows you to store a variable number of values next to each other.

• A string is a collection of characters. We’ve mentioned the String type
previously, but in this chapter we’ll talk about it in depth.

• A hash map allows you to associate a value with a particular key. It’s a particular
implementation of the more general data structure called a map.

*/
use std::collections::HashMap;


fn main() {

    // Vector Collection 
    // Initialize vector with type annotation
    let v: Vec<i32> = Vec::new();

    // a vector containing value which rust know what type it is
    let x = vec![1,2,3];

    // updating vector
    let mut z = Vec::new();
    // inserting these value into vector
    z.push(1);
    z.push(2);
    z.push(3);


    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {

        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
}

    // using reference
    // this will cause panic because it reference nothing
    let does_not_exist = &v[100];
    // using get method
    // this will get the None without panicking
    let does_not_exist = v.get(100);
    
    // Iterations over vector
    for i in &v {
        println!("{}", i);
    }
    // Iterating over mutable references to elements in a vector
    let mut v = vec![100, 32, 57];
        for i in &mut v {
        // Using * derefence operator
         *i += 50;
    }

    // Using Enum to Store Multiple Values
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


} // x will out of scope and is freed here


fn string_collection() {
    // -- SKIP --
}


fn hashmap_collection() {
    // Storing Keys with Associated Values in Hash Maps
    /*
    It does this via a hashing
    function, which determines how it places these keys and values into memory.
    Many programming languages support this kind of data structure, but they
    often use a different name, such as hash, map, object, hash table, dictionary,
    or associative array, just to name a few.
    */
    // HashMap<K, V>
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();



    /*
    We aren’t able to use the variables field_name and field_value after
    they’ve been moved into the hash map with the call to insert.
    If we insert references to values into the hash map, the values won’t be
    moved into the hash map. The values that the references point to must
    be valid for at least as long as the hash map is valid.
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{}", field_name);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {

    println!("{}: {}", key, value);

    }

    let getme = String::from("Blue");
    print("{}", scores.get(getme));

}