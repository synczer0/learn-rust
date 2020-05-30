
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

fn take_ownership(stra: i32) {
    println!("{}", stra);

}

fn main() {

   let s = String::from("Hello");
    let x = 10;
   take_ownership(x);

   println!("{}", x);
    

}
