
fn var_bindings() {
    // primitive integers
    // i8  - signed 8-bit
    // i16 - signed 16-bit
    // i32 - signed 32-bit
    // i64 - signed 64-bit
    // u8  - unsigned 8-bit
    // u16 - unsigned 16-bit
    // u32 - unsigned 32-bit
    // u64 - unsigned 64-bit
    // isize - signed size
    // usize  - unsigned size

    
    // assign 5 to x
    let x = 5;

    // Pattern
    // After this statement is evaluated, x will be one, and y will be two
    let (x, y) = (1, 2);

    // Type Annotations
    // If I asked you to read this out loud to the rest of the class, you’d
    // say “x is a binding with the type i32 and the value five.”
    let x: i32 = 5;

    // Mutability
    // By default, bindings are "immutable"
    
    // If we compile this it will give you an error
    let x = 5;
    x = 10;

    // If we want our bindings to become mutable, we can use "mut"
    let mut x = 5;
    x = 5;

    // Initialize Bindings
    // When we declared a variable but it is not initalize the compiler warn or give us an error
    // Every variable must initialize before using them
    let x: i32;

    // Tuples bindings
    // Tuples are immutable type, that does not changed on runtime
    let rec = (30, 40);
    // to access the tuple index we might use the . with number after it (.0 or .1 and so on)
    rec.0 // access 30 value
    rec.1 // access 40 value


    /*
    The double colon (::) is an operator that allows us to namespace this
    particular from function under the String type rather than using some sort
    of name like string_from.
    */

    // String is heap allocated, growable and not null terminated.
    // Create an empty and growable `String`
    String test = String::new();

    // Heap allocate a string
    String test = String::new("Quick brown fox");

}