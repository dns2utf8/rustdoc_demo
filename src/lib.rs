#![feature(doc_cfg)]

//! This is used to demo all the rustdoc features
//!
//! # Example
//!
//! ```
//! assert_eq!(4, 2 + 2);
//! ```

pub extern crate rand;
pub extern crate sudo as super_sudo;

/// Outer mod comment
pub mod world_mod {

    #[derive(Debug)]
    pub struct WorldStruct {
        i: usize,
    }
    impl WorldStruct {
        pub fn new() -> WorldStruct {
            WorldStruct { i: rand::random() }
        }
    }
}
pub mod mod1;
pub mod mod2;
pub mod mod3;

//pub use world_mod;
pub use sudo as super_user;
pub use world_mod::WorldStruct as World;

pub fn main() {
    let world = World::new();
    println!("Hello, {:?}!", world);
}

pub fn another() {
    todo!()
}
pub fn before() {
    unimplemented!()
}
pub fn chained() {
    unimplemented!()
}

/// short description
pub fn short() {
    unimplemented!()
}
/// tested description with `assert!(true)`
///
/// ```
/// assert!(true);
/// ```
pub fn tested() {
    unimplemented!()
}

/// One Two Three Four Five Six Seven Eight Nine Ten Eleven Twelve Thirteen Fourteen Fifteen Sixteen Seventeen Eighteen Nineteen Twenty
pub fn text_that_is_longer() {
    unimplemented!()
}

/// Closing Text
pub fn quit() {
    unimplemented!()
}

/// Unsafe Function declared first
pub unsafe fn unsafe_function2() {
    unimplemented!()
}
/// Unsafe Function declared second
pub unsafe fn unsafe_function1() {
    unimplemented!()
}

/// Last function in code
pub fn last_function() {
    todo!()
}
