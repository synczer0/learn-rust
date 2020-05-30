// Ownership
/*
 ### KEEP THIS RULES IN MIND ###

• Each value in Rust has a variable that’s called its owner.
• There can be only one owner at a time.
• When the owner goes out of scope, the value will be dropped.

*/

{
    // Scope
    let x = "Hello";
    // we can use 'x' here

}  // x will be no valid here. it goes out of scope


