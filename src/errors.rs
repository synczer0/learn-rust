// collection of errors
// Rust does not have exceptions like in other programming language



/**
 *  Windows Command for Backtrace
 *  CMD = 'set RUST_BACKTRACE=1'
 *  Powershell = $Env:RUST_BACKTRACE=1
 * 
 *  Linux Command for Backtrace
 *  export RUST_BACKTRACE=1
 * 
 *  then run 'cargo run' you will able to see the backtraces of rust program
 */


// call panic
panic!("crash and burn")

// $RUST_BACKTRACE=1 cargo run
/**
 * A backtrace is a list of all the functions that have been called to get to
this point.
 
 */



 // T and E are generic types
 /**
    Option<T>
     - Some()
     - None()

    Result<T, E>
     - Ok()
     - Err()


    let f = File::open("hello.txt");
    let _x = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("{}", error);
        }
    }; 

    What you need to know right now is that T represents
    the type of the value that will be returned in a success case within the Ok
    variant, and E represents the type of the error that will be returned in a
    failure case within the Err variant. 
  */
 // Recoverable errors using Result<T, E>

 // Example
use std::fs::File;
use std::io::{ErrorKind};

 fn main() {
    // the File::open function is an enum type Result<T, E>


    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
        panic!("Problem opening the file: {:?}", error)
        },
    };

    // Matching different errors
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
         other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

 }