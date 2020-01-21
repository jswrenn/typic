//! The byte-level representation of a type.

pub mod coproduct;
pub mod ops;
pub mod product;
pub mod slot;

pub use ops::{Add, Sum};
pub use product::{Cons as PCons, Nil as PNil};
