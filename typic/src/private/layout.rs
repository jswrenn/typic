use crate::private::highlevel::{HighLevelOf, ReprAlignOf, ReprPackedOf, Type};
use crate::private::num::Unsigned;

mod aligned_to;

mod into_bytelevel;
mod padding;

pub use aligned_to::AlignedTo;
use into_bytelevel::IntoByteLevel;
use padding::PaddingNeededForField;
use crate::private::highlevel::Public;

/// The actual memory layout characteristics of `Self`.
pub trait Layout<Visibility=Public> {
    /// The actual alignment of `Self`.
    type Align: Unsigned;

    /// The actual size of `Self`.
    type Size: Unsigned;

    /// The byte-level representation of `Self`.
    type ByteLevel;
}

#[rustfmt::skip]
impl<T, Visibility> Layout<Visibility> for T
where
    T: Type,

    HighLevelOf<T>:
        IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
            Visibility,
        >,
{
    type Align =
        <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
            Visibility,
        >>::Align;

    type Size =
        <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
            Visibility,
        >>::Offset;

    type ByteLevel =
        <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
            Visibility,
        >>::Output;
}

#[cfg(test)]
mod test;
