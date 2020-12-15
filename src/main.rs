#![allow(warnings, unused)]

//! Primitive types, variables, methods, mutability

mod basics;

/// App entry point
fn main() {

    // this is a comment
    println!("Hello, world!");

    let args = std::env::args();
    println!("Args: {:?}", args);

    basics::more_examples();
}

