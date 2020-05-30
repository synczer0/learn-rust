// Ownership
/*
 ### KEEP THIS RULES IN MIND ###

• Each value in Rust has a variable that’s called its owner.
• There can be only one owner at a time.
• When the owner goes out of scope, the value will be dropped.

*/

{
    // Scope
    let x = "Hello";
    // we can use 'x' here

}  // x will be no valid here. it goes out of scope

// another example
{
    // instead of being called shallow copy, i'ts known as 'move'

    let s1 = String::from("Hello");
    // s1 still valid here
    let s2 = s1; // s1 move the value to s2
    // s1 can't be valid here
    println!("{}", s1);   // running this will result a compile error

}

// another example
{
    // If we do want to deeply copy the heap data of the String, not just the stack
    // data, we can use a common method called clone.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Date: Copy
    /*
        But this code seems to contradict what we just learned: we don’t have a
        call to clone, but x is still valid and wasn’t moved into y.
        The reason is that types such as integers that have a known size at compile
        time are stored entirely on the stack, so copies of the actual values are
        quick to make. That means there’s no reason we would want to prevent x
        from being valid after we create the variable y. In other words, there’s no difference
        between deep and shallow copying here, so calling clone wouldn’t do
        anything different from the usual shallow copying and we can leave it out.
    */
    let x = 10;
    let y = x;

    println!("{}", x);

    /*
        Here are some of the types that are Copy:

        • All the integer types, such as u32.
        • The Boolean type, bool, with values true and false.
        • The character type, char.
        • All the floating point types, such as f64.
        • Tuples, if they only contain types that are also Copy. For example, (i32,
        i32) is Copy, but (i32, String) is not.
    

    */


}


// another example
{
    // Ownership and Functions

    let s = String::from("Hello");  // s comes of scope

    take_ownership(s);              // s value's move into the function
                                    // s value here no longer available or valid here
    let x = 10;                     // x comes in scope

    make_copy(x);                   // x value move into the function
                                    // but i32 is Copy, so it's okay to still use x afterward.

    fn take_ownership(some_string: String) {
        // some_string comes into scope
        println!("{}", some_string);
    }   // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn make_copy(some_int: i32) {
        // some_integer comes into scope
        println!("{}", some_int);
    }   // Here, some_integer goes out of scope. Nothing special happens.

}                                   // Here, x goes out of scope, then s. But because s's value was moved, 
                                    // nothing special happen


// another example
{
    // Return Values and Scope

        // MAIN
      {
        let s1 = gives_ownership(); // gives_ownership moves its return
        // value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3

      } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
        // moved, so nothing happens. s1 goes out of scope and is dropped.



        fn gives_ownership() -> String { // gives_ownership will move its
            // return value into the function
            // that calls it
            let some_string = String::from("hello"); // some_string comes into scope
            some_string // some_string is returned and
            // moves out to the calling
            // function
         }

         // takes_and_gives_back will take a String and return one
        fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
      
        a_string // a_string is returned and moves out to the calling function
    }

}

// another example
{
    // References and Barrowing
    

    
}