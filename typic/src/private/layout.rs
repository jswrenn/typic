use crate::private::highlevel::{HighLevelOf, ReprAlignOf, ReprPackedOf, Type};
use crate::private::num::Unsigned;
use generic_array::ArrayLength;

mod aligned_to;

mod into_bytelevel;
mod padding;

use crate::private::highlevel::Public;
pub use aligned_to::AlignedTo;
use into_bytelevel::IntoByteLevel;
use padding::PaddingNeededForField;

/// The actual memory layout characteristics of `Self`.
pub trait Layout<Visibility = Public> {
    /// The actual alignment of `Self`.
    type Align: Unsigned;

    /// The actual size of `Self`.
    type Size: Unsigned + ArrayLength<u8>;

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

    <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
            Visibility,
        >>::Offset: ArrayLength<u8>,
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
