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

/// Indicates a type has no internal validity requirements.
///
/// The `Transparent` trait is used to indicate that a compound type does not
/// place any additional validity restrictions on its fields.
///
/// This trait can be implemented ***manually***:
/// ```
/// # use typic::docs::prelude::*;
/// #[typic::repr(C)]
/// pub struct Unconstrained {
///     wizz: u8,
///     bang: i8,
/// }
///
/// unsafe impl Transparent for Unconstrained {}
///
/// let _ : Unconstrained = u16::default().transmute_into();
/// ```
///
/// Or, ***automatically***, by marking the fields `pub`:
/// ```
/// # use typic::docs::prelude::*;
/// #[typic::repr(C)]
/// pub struct Unconstrained {
///     pub wizz: u8,
///     pub bang: i8,
/// }
///
/// let _ : Unconstrained = u16::default().transmute_into();
/// ```
///
/// If the fields are marked `pub`, the type cannot rely on any internal
/// validity requirements, as users of the type are free to manipulate its
/// fields via the `.` operator.
pub unsafe trait Transparent: Type {}

pub(crate) type HighLevelOf<T> = <T as Type>::HighLevel;
pub(crate) type ReprAlignOf<T> = <T as Type>::ReprAlign;
pub(crate) type ReprPackedOf<T> = <T as Type>::ReprPacked;
