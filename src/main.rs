
#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::fmt::Display;
#[allow(unused_imports)]
use std::io;
// access loops.rs file which consists of functions
mod loops;

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

fn main() {

    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}", map);
}