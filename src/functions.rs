
// function with return values
fn rtnValue() => i32 {
    // return 5
    5
}

// pass a parameter(int) signed 32bit
fn passValue(x: i32) => i32 {
    // return value of (x + 10)
    x + 10    
}

// void function that does not have return type
fn test() {
    println!("Hello");
    let z = 1;


    if z == 2 {
        println!("Invalid #!!!");
    } else if z == 1 {
        println!("Success!!!");
    } else {
        println!("NGHEKK");
    }
}




