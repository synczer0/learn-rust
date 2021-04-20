// Structure/Struct
/*
 struct - is a custom data type
that lets you name and package together
multiple related values that make up a
meaningful group. If you’re familiar with an
object-oriented language, a struct is like an object’s
data attributes.
 
*/

// Struct Example
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Here we create a function that return a 'User' instance with the given email and username
// Don't use this kind of Field Init 
fn build_user(email: String, username: String) -> User {
    User {
    email: email, // email is the same email? such a tedious work if this kind of variable
    username: username,
    active: true,
    sign_in_count: 1,
    }
}

// Luckily we have a Field Init Shorthand
// Recommended using if you have same name parameters in your Fields
fn build_user(email: String, username: String) -> User {
    User {
    email, // we have a fields of email, with the same name as the parameter
    username,
    active: true,
    sign_in_count: 1,
    }
}

// Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Refactoring with Structs: Adding More Meaning
struct Rectangle {
    width: u32,
    height: u32,
}

// Defining Methods
// Implementation Block - we can use multiple impl block with the same named structure
impl Rectangle {
    fn area(&self) -> u32 {
    self.width * self.height
    }
}

// Associated Functions
// Another useful feature of impl blocks is that we’re allowed to define
// functions within impl blocks that don’t take self as a parameter. These
// are called associated functions because they’re associated with the struct.
// They’re still functions, not methods, because they don’t have an instance
// of the struct to work with.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


// we borrow struct Rectangle which is immutable, so the main will continue using rect1 variable
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
fn main() {
    
    // We must instantiate the struct before using its methods or fields
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
     };

    // Struct Update Syntax
    // The syntax .. specifies that the remaining fields not
    // explicitly set should have the same value as the fields in the given instance.
     let user2 = User {

        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // Using struct update syntax to set new email and username values for a User
        // instance but use the rest of the values from the fields of the instance in the user1 variable 
        ..user1
    };

    //  to access the fields of struct where using . method
     println!("{}", user1.username);

     let rect1 = Rectangle { width: 30, height: 50;}

    //  error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // {:?} specifier output a format "Debug"
    // {:#?} another specifier which display like this
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50
    // }

     println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    
    );

    // Print out using implementation Block
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    
    );
    
     
}