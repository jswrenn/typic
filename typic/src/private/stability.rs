//! A type grammar for communicating layout guarantees.
//!
//! ```rust
//! use typic::{self, stability::*};
//!
//! #[typic::repr(C)]
//! #[derive(StableABI)]
//! struct Foo(u8, u16, u32);
//! ```

//use crate::private::layout::{self, Layout};
use crate::layout::Layout;
use crate::private::highlevel::Public;

/// Implements all stability restrictions on a type.
pub use typic_derive::StableABI;

/// Increasing the given aspect is a breaking change.
pub enum Upper {}

/// Decreasing the given aspect is a breaking change.
pub enum Lower {}

/// The directionality of the layout guarantee.
pub trait Direction: private::Sealed {}

impl Direction for Upper {}
impl Direction for Lower {}

pub trait Bound<Direction>
where
    Direction: self::Direction
{
    type Type: Layout;
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for Upper       {}
    impl Sealed for Lower       {}
}
