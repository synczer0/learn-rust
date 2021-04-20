
fn main () {

    enum IpAddrKind{
        V4,
        V6
    }

    // Enum with data types
    enum Student {
        name(String),
        age(i32),
        grade(u32),
        classroom(String, String),
        address {street: String, block: String, province: String }
     
    }

    // Another usage for enum is using implementation block
    impl Student {
        fn call(&self) {
            // define here
        }
    }

    let m = Student::name(String::from("Gilbert"));
    m.call();

    // Rust does not have null but it has a concept of present and absence of value like this enum Option
    /*
    The Option<T> enum is so useful that it’s even included in the prelude;
    you don’t need to bring it into scope explicitly. In addition, so are its variants:
    you can use Some and None directly without the Option:: prefix. The
    Option<T> enum is still just a regular enum, and Some(T) and None are still
    variants of type Option<T>.
    
    The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a
    generic type parameter, and we’ll cover generics in more detail in Chapter 10.
   <T> means the Some variant of the Option enum can hold one piece of data of any type.

    */
    // enum Option<T> {
    //     Some(T),
    //     None
    // }

    // example of enum Option
    let some_number = Some(5);
    let some_string = Some("a string");
    // Option here is a valid enum without implementing a enum Option code. 
    let absent_number: Option<i32> = None;
    

    // Match Control Flow Operator
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        }

    #[derive(Debug)]
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

let some_u8_value = 0u8;
    match some_u8_value {

        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ placeholder if the matching does no match any of the following above execute this match
        // this is like on switch type 
        _ => (),
}

    //  Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
         None => None,
         Some(i) => Some(i + 1),
        }
    }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

}