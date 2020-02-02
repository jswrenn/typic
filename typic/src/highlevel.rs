//! The byte-level representation of a type.

pub mod coproduct;
pub mod product;

pub use typenum::consts::*;
use crate::num::{Unsigned};

#[doc(inline)]
pub use coproduct::{Cons as CCons, Nil as CNil};
#[doc(inline)]
pub use product::{Cons as PCons, Nil as PNil};

pub type MinAlign = U1;
pub type MaxAlign = U536870912;

pub trait Type {
    /// `align(N)`
    type ReprAlign: Unsigned;

    /// `packed(N)`
    type ReprPacked: Unsigned;

    /// An abstract representation of the type's structure.
    type HighLevel;
}

pub(crate) type HighLevelOf<T> = <T as Type>::HighLevel;
pub(crate) type ReprAlignOf<T> = <T as Type>::ReprAlign;
pub(crate) type ReprPackedOf<T> = <T as Type>::ReprPacked;
