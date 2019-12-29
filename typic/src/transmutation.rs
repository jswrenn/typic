use crate::alignedto::AlignedTo;
use crate::layout::{Init, Layout, NonZero, Uninit};
use crate::transmute::Candidate;
use core::mem;
use frunk_core::hlist::*;
use frunk_core::Hlist;
use static_assertions::*;

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
        Self: Candidate<Candidate = Self> + Sized,
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
pub unsafe trait Invariants: Candidate {
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
    U: Candidate<Candidate = Self>,
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
    T: Layout,
    U: Layout,
    <U as Layout>::Slots: FromLayout<<T as Layout>::Slots>,
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
        let to = unsafe { mem::transmute::<&T, &<U as Candidate>::Candidate>(&from) };
        Self::check(to).map(|Valid| unsafe { Self::transmute_from_unchecked(from) })
    }
}

// U: TransmuteFrom<T> indicates that the bytes of any valid T
// correspond to the bytes of a valid instance of U.
pub trait FromLayout<T> {}

/// Base case.
impl FromLayout<HNil> for HNil {}

// U: TransmuteFrom<T> indicates that the bytes of any valid T
// correspond to the bytes of a valid instance of U.

///  `Init -> *`
impl<TR, U1, UR> FromLayout<Hlist![Init, ...TR]> for Hlist![U1, ...UR]
where
    U1: FromSlot<Init>,
    UR: FromLayout<TR>,
{
}

// An initialized byte may only be constructed from another initialized byte.
assert_impl_all!(Hlist![Init]: FromLayout<Hlist![Init]>);

// An initialized byte may not be constructed from an uninitialized byte.
assert_not_impl_any!(Hlist![Init]: FromLayout<Hlist![Uninit]>);

///  `NonZero -> *`
impl<TR, U1, UR> FromLayout<Hlist![NonZero, ...TR]> for Hlist![U1, ...UR]
where
    U1: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

// An nonzero byte may only be constructed from another initialized byte.
assert_impl_all!(Hlist![NonZero]: FromLayout<Hlist![NonZero]>);

// An nonzero byte may not be constructed from an initialized or uninitialized byte.
assert_not_impl_any!(
    Hlist![NonZero]: FromLayout<Hlist![Init]>,
    FromLayout<Hlist![Uninit]>
);

/// `Uninit -> *`
impl<TR, U1, UR> FromLayout<Hlist![Uninit, ...TR]> for Hlist![U1, ...UR]
where
    U1: FromSlot<Uninit>,
    UR: FromLayout<TR>,
{
}

// An uninitialized byte may be constructed from an initialized or uninitialized byte.
assert_impl_all!(
    Hlist![Uninit]: FromLayout<Hlist![Init]>,
    FromLayout<Hlist![Uninit]>
);

/// `*const T -> *const U`
impl<T1, TR, U1, UR> FromLayout<Hlist![*const T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: TransmuteFrom<T1>,
    UR: FromLayout<TR>,
{
}

assert_impl_all!(Hlist![*const u64]: FromLayout<Hlist![*const i64]>);

assert_not_impl_any!(Hlist![*const u64]: FromLayout<Hlist![*const i16]>);

/// `*mut T -> *const U`
impl<T1, TR, U1, UR> FromLayout<Hlist![*mut T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: TransmuteFrom<T1>,
    UR: FromLayout<TR>,
{
}

// A const pointer may be created from a mut pointer.
assert_impl_all!(Hlist![*const u64]: FromLayout<Hlist![*mut u64]>);

// A mut pointer may NOT be created from a const pointer.
assert_not_impl_any!(Hlist![*mut u64]: FromLayout<Hlist![*const u64]>);

/// `&'t mut T -> *const U`
impl<'t, T1, TR, U1, UR> FromLayout<Hlist![&'t T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: AlignedTo<T1>,
    U1: TransmuteFrom<T1>,
    UR: FromLayout<TR>,
{
}

// A smart pointer may be converted to a const pointer.
assert_impl_all!(Hlist![*const u64]: FromLayout<Hlist![&'static u64]>);

// A const pointer may NOT be converter to a smart pointer.
assert_not_impl_any!(Hlist![&'static u64]: FromLayout<Hlist![*const u64]>);

/// `&'t T -> &'u U`
impl<'t, 'u, T1, TR, U1, UR> FromLayout<Hlist![&'t T1, ...TR]> for Hlist![&'u U1, ...UR]
where
    't: 'u,
    U1: AlignedTo<T1>,
    U1: TransmuteFrom<T1>,
    UR: FromLayout<TR>,
{
}

// Pointers are convertible if their underlying types are convertible.
assert_impl_all!(Hlist![&'static u64]: FromLayout<Hlist![&'static i64]>);

// Pointers are not convertible if their underlying types aren't convertible.
assert_not_impl_any!(Hlist![&'static u64]: FromLayout<Hlist![&'static u16]>);

/// `&mut 't T -> &'u U`
impl<'t, 'u, T1, TR, U1, UR> FromLayout<Hlist![&'t mut T1, ...TR]> for Hlist![&'u U1, ...UR]
where
    't: 'u,
    U1: AlignedTo<T1>,
    U1: TransmuteFrom<T1>,
    UR: FromLayout<TR>,
{
}

// Pointers are convertible if their underlying types are convertible.
assert_impl_all!(Hlist![&'static u64]: FromLayout<Hlist![&'static mut i64]>);

// Pointers are not convertible if their underlying types aren't convertible.
assert_not_impl_any!(Hlist![&'static mut u64]: FromLayout<Hlist![&'static u16]>);

/// `&mut 't T -> *const U`
impl<'t, T1, TR, U1, UR> FromLayout<Hlist![&'t mut T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: AlignedTo<T1>,
    U1: TransmuteFrom<T1>,
    UR: FromLayout<TR>,
{
}

// If the underlying types are convertible, a const pointer may be created from:
assert_impl_all!(
    Hlist![*const u64]: FromLayout<Hlist![*const i64]>,
    FromLayout<Hlist![*mut i64]>,
    FromLayout<Hlist![&'static i64]>,
    FromLayout<Hlist![&'static mut i64]>
);

// A mut smart pointer may not be created from a const pointer.
assert_not_impl_any!(Hlist![&'static mut u64]: FromLayout<Hlist![*const u64]>);

////////////////////////////////////////////////////////////////////////////////
// Pointer Decompositions
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_pointer_width = "64")]
type InitializedBytes<R = HNil> = Hlist![Init, Init, Init, Init, Init, Init, Init, Init, ...R];

#[cfg(target_pointer_width = "64")]
type NonZeroBytes<R = HNil> =
    Hlist![NonZero, NonZero, NonZero, NonZero, NonZero, NonZero, NonZero, NonZero, ...R];

#[cfg(target_pointer_width = "64")]
type UninitializedBytes<R = HNil> =
    Hlist![Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, ...R];

// pointers may be decomposed into initialized bytes
assert_impl_all!(
    InitializedBytes: FromLayout<Hlist![*const u64]>,
    FromLayout<Hlist![*mut u64]>,
    FromLayout<Hlist![&'static u64]>,
    FromLayout<Hlist![&'static mut u64]>
);

// pointers may be decomposed into uninitialized bytes
assert_impl_all!(
    UninitializedBytes: FromLayout<Hlist![*const u64]>,
    FromLayout<Hlist![*mut u64]>,
    FromLayout<Hlist![&'static u64]>,
    FromLayout<Hlist![&'static mut u64]>
);

// smart pointers may be decomposed into nonzero bytes
assert_impl_all!(
    NonZeroBytes: FromLayout<Hlist![&'static u64]>,
    FromLayout<Hlist![&'static mut u64]>
);

// raw pointers may NOT be decomposed into nonzero bytes
assert_not_impl_any!(
    NonZeroBytes: FromLayout<Hlist![*const u64]>,
    FromLayout<Hlist![*mut u64]>
);

////////////////////////////////////////////////////////////////////////////////
// Smart Pointer Decomposition
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_pointer_width = "64")]
/// &'t T -> `[Init; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<NonZero>,
    U3: FromSlot<NonZero>,
    U4: FromSlot<NonZero>,
    U5: FromSlot<NonZero>,
    U6: FromSlot<NonZero>,
    U7: FromSlot<NonZero>,
    U8: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t T -> `[NonZero; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t T, ...TR]> for Hlist![NonZero, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<NonZero>,
    U3: FromSlot<NonZero>,
    U4: FromSlot<NonZero>,
    U5: FromSlot<NonZero>,
    U6: FromSlot<NonZero>,
    U7: FromSlot<NonZero>,
    U8: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t T -> `[Uninit; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<NonZero>,
    U3: FromSlot<NonZero>,
    U4: FromSlot<NonZero>,
    U5: FromSlot<NonZero>,
    U6: FromSlot<NonZero>,
    U7: FromSlot<NonZero>,
    U8: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t mut T -> `[Init; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t mut T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<NonZero>,
    U3: FromSlot<NonZero>,
    U4: FromSlot<NonZero>,
    U5: FromSlot<NonZero>,
    U6: FromSlot<NonZero>,
    U7: FromSlot<NonZero>,
    U8: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t mut T -> `[NonZero; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t mut T, ...TR]> for Hlist![NonZero, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<NonZero>,
    U3: FromSlot<NonZero>,
    U4: FromSlot<NonZero>,
    U5: FromSlot<NonZero>,
    U6: FromSlot<NonZero>,
    U7: FromSlot<NonZero>,
    U8: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t mut T -> `[Uninit; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t mut T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<NonZero>,
    U3: FromSlot<NonZero>,
    U4: FromSlot<NonZero>,
    U5: FromSlot<NonZero>,
    U6: FromSlot<NonZero>,
    U7: FromSlot<NonZero>,
    U8: FromSlot<NonZero>,
    UR: FromLayout<TR>,
{
}

////////////////////////////////////////////////////////////////////////////////
// Raw Pointer Decompositions
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_pointer_width = "64")]
/// *const T -> `[Init; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*const T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<Init>,
    U3: FromSlot<Init>,
    U4: FromSlot<Init>,
    U5: FromSlot<Init>,
    U6: FromSlot<Init>,
    U7: FromSlot<Init>,
    U8: FromSlot<Init>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// *const T -> `[Uninit; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*const T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<Init>,
    U3: FromSlot<Init>,
    U4: FromSlot<Init>,
    U5: FromSlot<Init>,
    U6: FromSlot<Init>,
    U7: FromSlot<Init>,
    U8: FromSlot<Init>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// *mut T -> `[Init; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*mut T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<Init>,
    U3: FromSlot<Init>,
    U4: FromSlot<Init>,
    U5: FromSlot<Init>,
    U6: FromSlot<Init>,
    U7: FromSlot<Init>,
    U8: FromSlot<Init>,
    UR: FromLayout<TR>,
{
}

#[cfg(target_pointer_width = "64")]
/// *mut T -> `[Uninit; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*mut T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: FromSlot<Init>,
    U3: FromSlot<Init>,
    U4: FromSlot<Init>,
    U5: FromSlot<Init>,
    U6: FromSlot<Init>,
    U7: FromSlot<Init>,
    U8: FromSlot<Init>,
    UR: FromLayout<TR>,
{
}

////////////////////////////////////////////////////////////////////////////////
// Pointer Recompositions
////////////////////////////////////////////////////////////////////////////////

/// `[Init; target_pointer_width] -> *const U`
impl<TR, U1, UR> FromLayout<InitializedBytes<TR>> for Hlist![*const U1, ...UR] where
    UR: FromLayout<TR>
{
}

// const ptr may be created from initialized bytes
#[cfg(target_pointer_width = "64")]
assert_impl_all!(Hlist![*const u64]: FromLayout<InitializedBytes>);

// const ptr may NOT be created from uninitialized bytes
#[cfg(target_pointer_width = "64")]
assert_not_impl_any!(Hlist![*const u64]: FromLayout<UninitializedBytes>);

/// `[Init; target_pointer_width] -> *mut U`
impl<TR, U1, UR> FromLayout<InitializedBytes<TR>> for Hlist![*mut U1, ...UR] where UR: FromLayout<TR>
{}

// mut ptr may be created from initialized bytes
#[cfg(target_pointer_width = "64")]
assert_impl_all!(Hlist![*mut u64]: FromLayout<InitializedBytes>);

// mut ptr may NOT be created from uninitialized bytes
#[cfg(target_pointer_width = "64")]
assert_not_impl_any!(Hlist![*mut u64]: FromLayout<UninitializedBytes>);

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// A valid instance of `T` is also a valid instance of `Self`
pub trait FromSlot<T> {}

/// An initialized byte is a valid instance of an initialized byte.
impl FromSlot<Init> for Init {}

/// An nonzero byte is a valid instance of nonzero byte.
impl FromSlot<NonZero> for NonZero {}

/// An nonzero byte is a valid instance of an initialized byte.
impl FromSlot<NonZero> for Init {}

/// An nonzero byte is a valid instance of an uninitialized byte.
impl FromSlot<NonZero> for Uninit {}

/// An uninitialized byte is a valid instance of an uninitialized byte.
impl FromSlot<Uninit> for Uninit {}

/// An initialized byte is a valid instance of an uninitialized byte.
impl FromSlot<Init> for Uninit {}

/// `&'t T` is a valid instance of `&'u U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U> FromSlot<&'t T> for &'u U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `&'u U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U> FromSlot<&'t mut T> for &'u U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `&'u mut U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U> FromSlot<&'t mut T> for &'u mut U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}

/// `&'t T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<'t, T, U> FromSlot<&'t T> for *const U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<'t, T, U> FromSlot<&'t mut T> for *const U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `*mut U`, if the underlying types are
/// covertible.
impl<'t, T, U> FromSlot<&'t mut T> for *mut U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}

/// `*const T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<T, U> FromSlot<*const T> for *const U
where
    U: AlignedTo<T>,
    U: TransmuteFrom<T>,
{
}
