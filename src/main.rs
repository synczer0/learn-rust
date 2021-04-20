
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Display;
#[allow(unused_imports)]
use std::io;
// access loops.rs file which consists of functions
mod loops;

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(dead_code)]
fn func_name() {
    println!("HJello");

    let x = {
        let z = 0;
        z + 10
    };
    
    println!("{}", x);
}

#[allow(dead_code)]
fn take_ownership(stra: i32) {
    println!("{}", stra);

}

#[allow(dead_code)]
#[derive(Debug)]
struct User { 
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
enum Testaa {
    Hello,
    World(String),
    V4(String)
}

// enum Option<T> {
//     Some(T),
//     None
// }


#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    }

   
    #[derive(Debug)]
    #[allow(dead_code)]
    enum UsState {
        Alaska,
        Alabama,

        }

        #[allow(dead_code)]
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
    
                Coin::Penny => 1, 
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}", state);
                    25
                },
            }
        }

        
use std::fs::File;
use std::io::{ErrorKind};
fn main() {
  
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