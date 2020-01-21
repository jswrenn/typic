//! Compute the byte-level layout from a generic representation of a type.

use crate::num::U0;

pub mod field;
pub mod primitives;
pub mod product;

pub trait IntoByteLevel<Align, Packed, Offset = U0> {
    /// The byte-level representation of the type.
    type Output;

    /// The size of the type.
    type Offset;

    /// The actual alignment of the type.
    type Align;
}
