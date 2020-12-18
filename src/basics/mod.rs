//! Rust basics
//!
//! Primitive types, variables, methods, mutability
//!
//! # Examples
//!
//! add your examples here
//!

use std::cell::{Cell, RefCell};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::BuildHasher;
use std::iter::FromIterator;
use std::ops::Range;
use std::str::Bytes;

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
    let x = 42; // val
    let mut res = 0; // var
    res = x + y;
    res
}

// Not allowed!

// let x = 1;
// foo(new String("bar"));

/// Constant is allowed
const APP_ID: &str = "131343848";

/// Pure function
const fn const_fn(number: i32) -> i32 {
    return number + 64;
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
    //
    // Primitives
    //
    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    let x: i32 = i32::MAX;

    // let x2: i32 = i64::MAX;
    // let x2: i64 = i32::MAX;

    // upgrade is easy
    let y: i64 = x as i64; // cast 1
    let yy: i64 = x.into(); // cast 2
    let yyy: i64 = i64::from(x); // cast 3

    // let slice = &y.to_le_bytes()[..4];
    // let int_bytes: [u8;4]  = slice.try_into().expect("Can't convert to 4 len array");
    // let res = i32::from_le_bytes(int_bytes);
    // println!("From long {}", res);

    //
    // Unsafe
    //

    // downgrade is hard and dangerous
    unsafe {
        // let byte: i8 = std::mem::transmute(true);

        let int: [i32; 2] = std::mem::transmute(y);
        println!("i32 {:?}", int[0]);

        boom()
    }

    unsafe fn boom() {
        println!("inside unsafe function")
    }

    //
    // Arrays, Slices
    //

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

    //
    // String, &str
    //
    // https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices

    let str1: &str = "fat pointer to string";

    let str2 = "string in heap".to_string();
    let str3 = String::from("string in heap");

    // naming conventions
    // https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
    let bytes = str2.as_bytes(); // without copying
    let bytes = str2.to_lowercase(); // returns copy
    let bytes = str2.into_bytes(); // without copy in case of string, but no guaranties in general

    //
    // Tuples
    //

    let tuple = (1, "test", true);
    let (int, str, boolean) = tuple;
    let int = tuple.0;
    let str = tuple.1;
    let boolean = tuple.2;

    fn tuple_destruction((x, y): (i32, i64)) {
        println!("tuple_destruction {}", x);
    }

    //
    // Control flow
    //
    // https://doc.rust-lang.org/book/ch03-05-control-flow.html

    if 3 > 1 {
        "less"
    } else {
        "more"
    };

    for n in 1..10 {
        if n % 5 == 0 {
            println!("{}", n + 1)
        }
    }

    //
    // Collections
    //
    // https://doc.rust-lang.org/std/collections/index.html

    let mut vector = Vec::new();
    vector.push("1");
    vector.push("2");
    vector.push("3");
    // or
    let vector = vec![1, 2, 3];

    let mut map = HashMap::new();
    map.insert(1, "1");
    map.insert(2, "2");
}

/// * Each value in Rust has a variable that’s called its owner.
/// * There can only be one owner at a time.
/// * When the owner goes out of scope, the value will be dropped.
pub fn ownership_borrowing() {
    //
    // Ownership and move semantic
    //

    let str = String::from("!");

    {
        let int = 32;
    } // here we destroy `y`

    // let z = 10 + int; // cannot find value `y` in this scope

    fn test(string: String) {
        string.to_lowercase();
    } // string get destroyed here

    test(str); // moved to function

    // println!("{}", str); // use after move

    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move

    // primitives have copy semantic

    let x = 12;
    fn copy_int(x: i32) {}
    copy_int(x);
    println!("{}", x);

    let str = String::new();

    fn test2(x: String) -> String {
        // do something then take back
        x
    }

    let str = test2(str); // moved to function

    println!("{}", str); // ok

    //
    // Borrowing
    //

    let str = String::new();
    let str1 = &str;
    let str2 = &str;

    fn test_by_ref(x: &String) {}

    test_by_ref(str1);
    test_by_ref(str2);
    test_by_ref(&str);

    println!("i32: {}", str);

    let mut str_mut = String::from("mut string");

    fn test_by_ref_mut(x: &mut String) {
        x.push('!');
    }
    test_by_ref_mut(&mut str_mut);

    println!("Changed str {}", str_mut);

    // read write lock in types - no more data races !!! )))


    //
    // Lifetimes
    //
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
}

pub fn traits_structs_enums() {
    //
    // Structs
    //

    struct Unit;

    struct Pair(i32, Unit);

    struct User {
        pub name: String,
        age: usize,
    }

    let user = User {
        name: String::from("Rod"),
        age: 21,
    };

    let User { name, age } = user;

    let user = User { name, age };

    fn struct_destruction(User { name, age }: User) {
        println!("struct_destruction {}, {}", name, age);
    }

    impl User {
        const ZERO: usize = 21;

        fn new(name: String, age: usize) -> Self {
            User { name, age }
        }

        fn default() -> Self {
            User::new(String::from("default"), User::ZERO)
        }

        // copy of name
        fn to_name(&self) -> String {
            self.name.clone()
        }

        // destructs and give name without copying
        fn into_name(self) -> String {
            self.name
        }

        // give name by reference
        fn name(&self) -> &str {
            &self.name
        }
    }

    user.to_name();
    User::to_name(&user);

    //
    // Traits
    //

    trait AgeGetter {
        fn get(&self) -> usize;
    }

    trait AgeSetter {
        fn set(&mut self, num: usize);
    }

    trait AgeGetterSetter: AgeGetter + AgeSetter {}

    impl AgeGetter for User {
        fn get(&self) -> usize {
            self.age
        }
    }

    impl AgeSetter for User {
        fn set(&mut self, age: usize) {
            self.age = age
        }
    }

    // for types defined AgeGetter + AgeSetter automatic derived AgeManager
    impl<T: AgeGetter + AgeSetter> AgeGetterSetter for T {}

    let get_set: User = User::default();
    get_set.get();

    // [subtype polymorphism] about dyn - https://doc.rust-lang.org/std/keyword.dyn.html
    let get_set: &dyn AgeGetterSetter = &User::default();
    get_set.get();

    impl AgeGetter for Unit {
        fn get(&self) -> usize {
            todo!()
        }
    }

    impl AgeSetter for Unit {
        fn set(&mut self, num: usize) {
            todo!()
        }
    }

    let mut vec = Vec::<&dyn AgeGetterSetter>::new();
    vec.push(get_set);
    vec.push(&Unit);

    //
    // Generics
    //

    // [parametric polymorphism] about monopolization - https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
    fn generic<T>(value: T) -> T {
        todo!()
    }

    // [bounded parametric polymorphism = type classes] about dyn - https://doc.rust-lang.org/std/keyword.dyn.html
    fn print_age<T: AgeGetter>(value: T) {
        println!("{:?}", value.get())
    }

    fn many_params<A, B>(a: A, b: B)
    where
        A: AgeGetter + AgeSetter + Display + Clone,
        B: ToString + Clone + AgeGetterSetter,
    {
        todo!()
    }

    //
    // Associative types
    //

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    impl Iterator for Unit {
        type Item = ();
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    fn impl_trait() -> impl AgeSetter {
        User::default()
    }
    // about impl trait - https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html

    // doesn't know yet how to print
    // println!("{:?}", user);

    // impl Debug for User {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    //         write!(f, "User({}, {})", self.name, self.age)
    //     }
    // }

    //
    // Option
    //

    let option = Some(1);

    match option {
        Some(number) => "it's a number",
        None => "Not a number",
    };

    if let Some(num) = option {
        println!("{} is number", num)
    }

    //
    // References, Mutability
    //

    let user: User = User::default();
    // user.age = 10; // impossible, immutable variable

    let mut mut_user = User::default();
    mut_user.age = 10; // ok

    let user_ref = &user;
    let user_ref2 = &user;
    let user_ref3 = &user;

    println!("{} {} {}", user_ref.age, user_ref2.age, user_ref3.age);

    // let user_ref_mut = &mut user; // can't get mut reference from immutable variable
    // user_ref.set(45);             // impossible, immutable variable

    let user_ref_mut = &mut_user;
    let user_ref_mut2 = &mut_user;
    let user_ref_mut3 = &mut_user;

    println!(
        "{} {} {}",
        user_ref_mut.age, user_ref_mut2.age, user_ref_mut2.age
    );

    mut_user.set(12); // ok

    let user_ref_mut = &mut mut_user; // first mut ref
    user_ref_mut.set(13); // ok

    let user_ref_mut_2 = &mut mut_user; // second mut ref, wtf?
    user_ref_mut_2.set(14); // ok

    mut_user.set(15); // ok,  wtf?

    // user_ref_mut.set(15);     // can't create more than 1 mutable reference

    // For interior mutability uses Cell, RefCell, Arc etc.
    // read more here - https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#refcellt-and-the-interior-mutability-pattern

    #[derive(Debug)]
    struct Metric {
        name: String,
        counter: Cell<usize>,
    }

    let metric = Metric {
        name: String::from("request total"),
        counter: Cell::new(0),
    };

    println!("Metric before: {:?}", metric);

    metric.counter.update(|counter| counter + 1);
    metric.counter.update(|counter| counter + 1);
    metric.counter.update(|counter| counter + 1);

    println!("Metric after: {:?}", metric);
}

pub fn function_closures() {
    //
    // Closures
    //
    // https://cheats.rs/#closures-in-apis

    let mut vec = vec![1, 2, 3];

    // function, just a pointer
    let func: fn(i32) -> i32 = |x: i32| x;

    let clojure: &dyn Fn(i32) -> bool = &|x: i32| *{ &vec.is_empty() };

    let clojure_once: &dyn FnMut(i32) -> () = &|x: i32| {
        &mut vec.push(1);
        ()
    };

    let clojure_mut: &dyn FnOnce(i32) -> () = &|x: i32| drop(vec);
}
