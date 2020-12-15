//! Rust basics
//!
//! Primitive types, variables, methods, mutability
//!
//! # Examples
//!
//! add your examples here
//!

use std::collections::HashMap;

// Primitives

fn foo(bar: String) {
    println!("foo-{}", bar)
}

/// Some example method
///
/// ```ignored
///     assert_eq!("foo", "foo")
/// ```
pub fn echo(y: i32) -> i32 {
    let x = 42;      // val
    let mut res = 0; // var
    res = x + y;
    return res;
}

// Not allowed!

// let x = 1;
// foo(new String("bar"));

// Constant is allowed
const APP_ID: &str = "131343848";

/// Pure function
const fn const_fn(number: i32) -> i32 {
    return number + 64
}
pub const APP_ID_2: i32 = const_fn(32);

// Not allowed
// const MAP: HashMap<&'static str, i32> = {
//     let mut map = HashMap::new();
//     map.insert("k1", 1);
//     map.insert("k2", 2);
//     map.insert("k3", 3);
//     map
// };

// if you want more use library [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)


pub fn more_examples() {

    // Primitives https://doc.rust-lang.org/book/ch03-02-data-types.html

    let x: i32 = i32::MAX;

    // let x2: i32 = i64::MAX;
    // let x2: i64 = i32::MAX;

    // upgrade is easy
    let y: i64 = x as i64; // cast 1
    let yy: i64 = x.into(); // cast 2
    let yyy: i64 = i64::from(x); // cast 3

    // Unsafe

    // downgrade is hard and dangerous
    unsafe {
        // let byte: i8 = std::mem::transmute(true);

        let int: [i32; 2] = std::mem::transmute(y);
        println!("i32 {:?}", int);

        boom()
    }

    unsafe fn boom() {
        println!("inside unsafe function")
    }

    // Arrays, Slices

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    array_fn(arr);

    fn array_fn(arr: [i32; 5]) {
        println!("array is {:?}", arr)
    }

    let slice = &arr[..]; // pointer with size
    slice_fn(&arr[1..3]);

    fn slice_fn(arr: &[i32]) {
        println!("slice &arr[1..3] is {:?}", arr)
    }

    // String, &str


}

