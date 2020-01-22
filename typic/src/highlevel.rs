//! The byte-level representation of a type.

pub mod coproduct;
pub mod product;

pub use crate::num::Unsigned;
pub use product::{Cons as PCons, Nil as PNil};

pub trait Type {
    /// `align(N)`
    // TODO: Rename to `ReprAlign`.
    type Align: Unsigned;

    /// `packed(N)`
    // TODO: Rename to `ReprPacked`.
    type Packed: Unsigned;

    /// An abstract representation of the type's structure.
    type HighLevel;
}

pub type HighLevelOf<T> = <T as Type>::HighLevel;
pub type ReprAlignOf<T> = <T as Type>::Align;
pub type ReprPackedOf<T> = <T as Type>::Packed;
