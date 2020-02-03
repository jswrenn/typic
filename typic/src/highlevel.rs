//! Types for encoding the high-level representation of a type's structure.

pub mod coproduct;
pub mod product;

use crate::num::Unsigned;

#[doc(hidden)]
pub use typenum::consts::*;

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

/// A user-defined type is `Transparent` its validity requirements are no
/// stricter than those of its fields.
///
/// This trait is implemented automatically by `#[typic::repr(...)]` for types
/// whose fields are all marked `pub`.
pub trait Transparent: Type {}

pub(crate) type HighLevelOf<T> = <T as Type>::HighLevel;
pub(crate) type ReprAlignOf<T> = <T as Type>::ReprAlign;
pub(crate) type ReprPackedOf<T> = <T as Type>::ReprPacked;
