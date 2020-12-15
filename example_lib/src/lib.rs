//!
//! # Examples
//!
//!```compile_fail
//!
//!  let me_speak: i32 = from - my + heart;
//!```
//!
//! ```
//!  let s = "Hello".to_string();
//!  let message = s + " world!";
//! ```
//!
//! [document test](https://doc.rust-lang.org/rustdoc/documentation-tests.html)
//! [works only for libs](https://github.com/rust-lang/rust/issues/50784)


/// My supper methods
///
/// ```
///   # use rust_introduction::foo;
///
///   let input = "test string";
///   let res = foo(input);
///   assert_eq!(res, input)
/// ```
pub fn foo(bar: &str) -> &str {
    return bar
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn foo_test() {
        let input = "test string";
        let res = foo(input);
        assert_eq!(res, input)
    }
}
