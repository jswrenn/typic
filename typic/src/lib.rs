pub use typic_derive::repr;

mod alignedto;
mod hlist;
mod layout;
mod transmutation;

/// Types used to represent the structure of compound types.
pub mod structure {
    use core::marker::PhantomData;

    /// A type-level linked list representing a struct's fields.
    /// `F` is the type of the first field.
    /// `R` is the type of the list representing the remaining fields.
    pub struct Fields<F, R>
    where
        R: FieldList,
    {
        data: PhantomData<(F, R)>,
    }

    /// The end of a type-level linked list.
    pub struct Empty;

    pub trait FieldList {}

    impl<F, R> FieldList for Fields<F, R> where R: FieldList {}

    impl FieldList for Empty {}
}

/// Marker types for the padding mode of compound types.
pub mod padding {
    /// A marker indicating that a compound type is `#[repr(packed)]`
    pub struct Packed;

    /// A marker indicating that a compound type is not `#[repr(packed)]`.
    pub struct Padded;

    /// A trait defining the set of possible padding modes.
    pub trait Padding {}

    impl Padding for Packed {}

    impl Padding for Padded {}
}

/// Types for safe transmutation.
///
/// ## Examples
/// ### Unrestricted and Restricted Transmutations
/// ```
/// use typic::{self, transmute::{Invariants, Valid, TransmuteFrom}};
///
/// #[typic::repr(C)]
/// #[derive(Default)]
/// struct Struct1 {
///   a: u16,
///   b: u16,
/// }
///
/// // If all fields are public, it is assumed that there are no additional
/// // invariants placed on the fields beyond what they individually have.
/// #[typic::repr(C)]
/// #[derive(Default)]
/// struct Struct2 {
///   pub a: u16,
///   pub b: u8,
///   pub c: u8,
/// }
///
/// // We can transmute safely and without checks from `Struct1` to `Struct2`.
/// let _ = Struct2::transmute_from(Struct1::default());
///
/// // Let's place some invariants on `Struct`.
/// unsafe impl Invariants for Struct1 {
///     type Error = &'static str;
///
///     #[inline(always)]
///     fn check(candidate: &Self::Candidate) -> Result<Valid, Self::Error>
///     where
///         Self: Sized,
///     {
///         if candidate.a % 2 == 0 {
///           Ok(Valid)
///         } else {
///           Err("`a` must be even")
///         }
///     }
/// }
///
/// // Now let's try to go in the other direction:
/// assert!(Struct1::try_transmute_from(Struct2 {a: 0, b: 0, c: 0}).is_ok());
/// assert!(Struct1::try_transmute_from(Struct2 {a: 1, b: 0, c: 0}).is_err());
/// ```
///
/// ### Lifetime Contraction
/// ```
/// use static_assertions::*;
/// use typic::{self, transmute::{Invariants, Valid, TransmuteFrom}};
///
/// fn contract<'long, 'short>(long: &'long u8) -> &'short u8
/// where 'long: 'short
/// {
///   TransmuteFrom::<&'short u8>::transmute_from(long)
/// }
/// ```
/// 
/// ### Lifetime Expansion
/// Typic cannot be used to expand lifetimes. This produces a compilation error:
/// ```compile_fail
/// use static_assertions::*;
/// use typic::{self, transmute::{Invariants, Valid, TransmuteFrom}};
///
/// fn expand<'short>(short: &'short u8) -> &'static u8
/// {
///   <&'static u8 as TransmuteFrom::<&'short u8>>::transmute_from(short)
/// }
/// ```
pub mod transmute {
    pub use crate::transmutation::{Invariants, TransmuteFrom, Valid};

    /// A candidate of a type is a doppelganger sharing that type's structure, but
    /// not its methods or invariants.
    pub trait Candidate {
        type Candidate;
    }
}

/// A generic representation of a type.
pub trait Type: transmute::Candidate {
    /// The padding mode of the type.
    type Padding: padding::Padding;

    /// An abstract representation of the type's structure.
    type Representation;
}
