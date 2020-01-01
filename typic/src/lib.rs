#![feature(marker_trait_attr)]

use static_assertions::*;
pub use typic_derive::repr;

pub mod hir;
pub mod hir_into_mir;
pub mod mir;
pub mod mir_convert;

/// A valid instance of `T` is also a valid instance of `Self`
pub unsafe trait TransmuteFrom<T> {
    /// Unsafe conversion from `T` to `U`.
    ///
    /// Construct an instance of `Self` from `T`, **without** ensuring that
    /// user-defined layout invariants of `T` are satisfied. This conversion
    /// **will** ensure that compiler-defined layout invariants are satisfied.
    ///
    /// It is not undefined behavior to use this function to construct
    /// instances of `Self` when `Self` has user-defined layout invariants on
    /// its members, but subsequently invoking procedures on `Self` that expect
    /// those layout variants to be satisfied may introduce undefined behavior.
    unsafe fn transmute_from_unchecked(from: T) -> Self
    where
        Self: Sized;

    /// Safe, infallible conversion from `T` to `U`.
    ///
    /// Construct an instance of `Self` from `T`, ensuring that both compiler-
    /// defined and user-defined layout invariants are satisfied.
    // -------------------------------------------------------------------------
    // This member is `inline(always)`, as it should be a zero-cost, safe
    // abstraction over `transmute_from_unchecked`.
    #[inline(always)]
    fn transmute_from(from: T) -> Self
    where
        Self: hir::Candidate<Candidate = Self> + Sized,
    {
        unsafe { Self::transmute_from_unchecked(from) }
    }

    /// Safe, fallible conversion from `T` to `U`.
    ///
    /// Construct an instance of `Self` from `T`, ensuring that user-defined
    /// layout invariants of `T` are satisfied.
    // -------------------------------------------------------------------------
    // This is implemented as a member of `TransmuteFrom` because end-users cannot
    // write implementations of `TransmuteFrom`—it has a blanket implementation.
    // This member becomes available upon a user's implementation of
    // `Invariants`, in which they implement `check`.
    fn try_transmute_from(from: T) -> Result<Self, <Self as Invariants>::Error>
    where
        Self: Invariants + Sized;
}

/// A result type for indicating conversions that are valid.
pub struct Valid;

/// A trait indicating that instances of a type `T` are bit-valid instances of a
/// type `U`, **if they satisfy the invariants of `check`**.
pub unsafe trait Invariants: hir::Candidate {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Produces `Valid` if `&to` is a valid instance of `Self`, otherwise
    /// produces `Error`.
    fn check(to: &Self::Candidate) -> Result<Valid, Self::Error>
    where
        Self: Sized;
}

/// A type has no invariants if its candidate type is equal to `Self`.
unsafe impl<U> Invariants for U
where
    U: hir::Candidate<Candidate = Self>,
{
    /// If a type `U` is `Arbitrary`, then conversions of `T` to `U` are
    /// infallible.
    type Error = core::convert::Infallible;

    /// If a type `U` is `Arbitrary`, then conversions of `T` to `U` are
    /// infallible. Therefore, this _always_ produces `Ok(Valid)`.
    // ------------------------------------------------------------------------
    // This member is `inline(always)`, as it should be a no-op.
    #[inline(always)]
    fn check(_: &Self::Candidate) -> Result<Valid, Self::Error>
    where
        Self: Sized,
    {
        Ok(Valid)
    }
}

/// Implement `TransmuteFrom<T>` for `U`, for layout-compatible `T` and `U`.
unsafe impl<T, U> TransmuteFrom<T> for U
where
    T: hir_into_mir::Layout,
    U: hir_into_mir::Layout,
    <U as hir_into_mir::Layout>::Representation:
        mir_convert::FromLayout<<T as hir_into_mir::Layout>::Representation>,
{
    #[inline(always)]
    unsafe fn transmute_from_unchecked(from: T) -> Self
    where
        Self: Sized,
    {
        let to = core::mem::transmute_copy(&from);
        core::mem::forget(from);
        to
    }

    #[inline(always)]
    fn try_transmute_from(from: T) -> Result<Self, <Self as Invariants>::Error>
    where
        Self: Invariants + Sized,
    {
        // Construct a candidate of `U`.
        let to = unsafe { core::mem::transmute::<&T, &<U as hir::Candidate>::Candidate>(&from) };
        Self::check(to).map(|Valid| unsafe { Self::transmute_from_unchecked(from) })
    }
}

mod test {
    pub use static_assertions::*;
    pub use typenum::*;
    pub use typic_derive::typicrepr;

    pub mod typic {
        pub use crate::*;
    }

    #[typicrepr(C)]
    union Test {
        a: u8,
    }

    assert_impl_all!(Test: crate::hir_into_mir::Layout);
}

//impl hir::Type for Test {
//  type Padding = hir::padding::Padded;
//
//  type Representation =
//    hir::coproduct::Cons<
//      i8,
//      hir::coproduct::Nil,
//    >;
//}
//
//
//
//assert_impl_all!(Foo: TransmuteFrom<Bar>);
//
//
////assert_impl_all!(Test: TransmuteFrom<Bar>);
//assert_impl_all!(Test: TransmuteFrom<Test>);
//assert_impl_all!(Test: TransmuteFrom<Bar>);
//assert_impl_all!(Bar: TransmuteFrom<Test>);
//
//
