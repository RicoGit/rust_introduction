#![allow(warnings, unused)]
#![feature(cell_update)] // enable unstable feature

//! Primitive types, variables, methods, mutability

mod basics;

/// App entry point
fn main() {
    // this is a comment
    println!("Hello, world!");

    let args = std::env::args();
    println!("Args: {:?}", args);

    basics::more_examples();
    basics::traits_structs_enums();
    basics::ownership_borrowing();
}
