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
use crate::transmute::{self, neglect, TransmuteFrom, TransmuteInto};
use crate::private::highlevel::Public;

/// Implements all stability restrictions on a type.
pub use typic_derive::StableABI;

/// `Self` is always transmutable into `Type`.
pub trait TransmutableInto
//  : Bound<Lower>
//  + TransmuteFrom<<Self as Bound<Lower>>::Type, neglect::Stability>
{
    type Type: Layout;
}

/// `Self` is always transmutable from `Type`.
pub trait TransmutableFrom
//  : Bound<Upper>
//  + TransmuteInto<<Self as Bound<Lower>>::Type, neglect::Stability>
{
    type Type: Layout;
}
