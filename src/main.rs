
#[allow(unused_imports)]
use std::fmt;

// access loops.rs file which consists of functions
mod loops;

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


fn main() {

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
     };

     

     println!("{:?}", user1.username);
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}