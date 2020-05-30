
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

fn main() {

    let mut s = String::from("Hello");

    s.push_str(" World!");
    s.remove(3);
    println!("{}", s);

}
