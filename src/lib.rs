//! This is used to demo all the rustdoc features
//!
//! # Example
//!
//! ```
//! assert_eq!(4, 2 + 2);
//! ```

mod world_mod {
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

pub use rand;
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
/// tested description
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
