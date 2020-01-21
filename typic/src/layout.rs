use crate::highlevel::{HighLevelOf, ReprAlignOf, ReprPackedOf, Type};
use crate::num::Unsigned;

pub mod into_bytelevel;
pub mod padding;

pub use into_bytelevel::IntoByteLevel;
pub use padding::PaddingNeededForField;

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
