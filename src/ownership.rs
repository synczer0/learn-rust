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
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1); // &s1 referers or point to the value of s1, it will not be dropped 
                                         // when reference goes out of scope

        change(&s1); // This will result an error because it is immutable
        
        change(&mut s1); // This will run because it is mutable

        println!("The length of '{}' is {}.", s1, len);



        // The benefit of having this restriction is that Rust can prevent data races
        // at compile time. A data race is similar to a race condition and happens when
        // these three behaviors occur:
        // • Two or more pointers access the same data at the same time.
        // • At least one of the pointers is being used to write to the data.
        // • There’s no mechanism being used to synchronize access to the data.

        // mutable references have one big restriction: you can have only one
        // mutable reference to a particular piece of data in a particular scope.
        let mut s = String::from("hello");
        let r1 = &mut s; // first mutable, ok it can be used since its only 1 mutable
        let r2 = &mut s; // second mutable, this is not ok, you have now 2 mutable which is against the rules
        
        println!("{}, {}", r1, r2); // can't execute this code will fail



    }

    // immutable reference
    fn change(some_string: &String) {
        some_string.push_str(", add") // cannot be add because it is immutable while borrowing
    }

    // mutable reference
    fn change(some_string: &String) {
        some_string.push_str(", add") // cannot be add because it is immutable while borrowing
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }// Here, s goes out of scope. But because it does not have ownership of
    // what it refers to, nothing happens.

    
}

// another example
{
    // dangling reference

    
    fn main() {
        let reference_to_nothing = dangle();
    }

    // this function's return type contains a borrowed value, but there is
    // no value for it to be borrowed from.
    fn dangle() -> &String { // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String
        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!!!!

}

// another example
{
    // Slice
    let s = String::from("Hello World");
    // .. is a ranged syntax
    let hello = &s[0..5];  // 0 is the starting index and 5 is the last index to slice
    let world = &s[6..11]; // 6 is the starting index and 11 is the last index to slice

    let slice = &s[..3];
    let getLength = s.len();

    // This both is equal if getting the whole string
    let slice = &s[0..len];
    let slice = &s[..];

    let slice = &s[0..getLength]; // this is also valid since we get the length of the string

    let intSlice = [1,2,3,4,5];

    let slice = &intSlice[0..2]; // we can also slice using type i32

    // &str - string slices
    // using &str (string slice) it allows us to use the same function on both String and &str values.

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item)v in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        &s[..]
    }


    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item)v in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }



}