//! Types for encoding the high-level representation of a type's structure.

pub mod coproduct;
pub mod field;
pub mod product;

use crate::private::num::Unsigned;

#[doc(hidden)]
pub use typenum::consts::*;

#[doc(inline)]
pub use field::{Field, Private, Public};

#[doc(inline)]
pub use coproduct::{Cons as CCons, Nil as CNil};
#[doc(inline)]
pub use product::{Cons as PCons, Nil as PNil};

pub type MinAlign = U1;
pub type MaxAlign = U536870912;

/// Implemented for types whose structure is understood by typic.
///
/// This trait is implemented for [many primitive types](#foreign-impls) and for
/// user-defined types annotated with the `#[typic::repr(...)]` attribute. This
/// trait should **not** be implemented manually.
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
