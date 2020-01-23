use crate::highlevel::{HighLevelOf, ReprAlignOf, ReprPackedOf, Type};
use crate::num::Unsigned;

mod into_bytelevel;
mod padding;

use into_bytelevel::IntoByteLevel;
use padding::PaddingNeededForField;

/// The actual memory layout characteristics of `Self`.
pub trait Layout {
    /// The actual alignment of `Self`.
    type Align: Unsigned;

    /// The actual size of `Self`.
    type Size: Unsigned;

    /// The byte-level representation of `Self`.
    type ByteLevel;
}

#[rustfmt::skip]
impl<T> Layout for T
where
    T: Type,

    HighLevelOf<T>:
        IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
        >,
{
    type Align =
        <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
        >>::Align;

    type Size =
        <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
        >>::Offset;

    type ByteLevel =
        <HighLevelOf<T> as IntoByteLevel<
            ReprAlignOf<T>,
            ReprPackedOf<T>,
        >>::Output;
}
