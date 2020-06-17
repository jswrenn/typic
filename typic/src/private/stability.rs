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

/// Implements [`TransmutableInto`] and [`TransmutableFrom`] for a
/// type, using that type as its own ABI bound.
///
/// You must not make any changes to this type that narrows the
/// visibility of its fields or changes its layout.
pub use typic_derive::StableABI;

/// Assert that `Self` is always transmutable into `Type`.
pub unsafe trait TransmutableInto
{
    type Type: Layout;
}

/// Assert that `Self` is always transmutable from `Type`.
pub unsafe trait TransmutableFrom
{
    type Type: Layout;
}
