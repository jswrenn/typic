use crate::hir::Candidate;
use crate::hir_into_mir::Layout;
use crate::mir::*;

use core::mem;
use static_assertions::*;
use typenum::PartialDiv;

macro_rules! Hlist {
    () => { product::Nil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { Hlist![$A,] };
    ($A:ty, $($tok:tt)*) => {
        product::Cons<$A, Hlist![$($tok)*]>
    };
}

macro_rules! Coprod {
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { coproduct::Nil<$A> };
    ($A:ty, $($tok:tt)*) => {
        coproduct::Cons<$A, Coprod![$($tok)*]>
    };
}

/// `U: AlignedTo<T>` indicates that `U`’s alignment requirement is at least as
/// strict as `T`'s, and so any memory address which satisfies the alignment of
/// `U` also satisfies the alignment of `T`.
pub trait AlignedTo<T> {}

impl<T, U> AlignedTo<T> for U
where
    T: Layout,
    U: Layout,
    <T as Layout>::Align: PartialDiv<<U as Layout>::Align>,
{
}

// U: crate::TransmuteFrom<T> indicates that the bytes of any valid T
// correspond to the bytes of a valid instance of U.
#[marker]
pub trait FromLayout<T> {}

// every T in Coprod![T, ...TR] has a valid variant in Coprod![U, ...UR]
impl<T, TR, U, UR> FromLayout<Coprod![T, ...TR]> for Coprod![U, ...UR]
where
    Self: FromLayout<T>,
    Self: FromLayout<TR>,
    T: Representation,
    U: Representation,
    TR: Coproduct,
    UR: Coproduct,
{
}

// every T in Coprod![T, ...TR] has a valid variant in Coprod![U, ...UR]
impl<T, U, UR> FromLayout<Coprod![T]> for Coprod![U, ...UR]
where
    Self: FromLayout<T>,
    T: Representation,
    U: Representation,
    UR: Coproduct,
{
}

impl<T, U> FromLayout<T> for Coprod![U] where U: Representation + FromLayout<T> {}

/* OVERLAPPING IMPLS START HERE */
// (A) `T` -> `U`
impl<T, U, UR> FromLayout<T> for Coprod![U, ...UR]
where
    U: Representation + FromLayout<T>,
    UR: Coproduct,
{
}

// (B) `T` -> something in `UR`
impl<T, U, UR> FromLayout<T> for Coprod![U, ...UR]
where
    U: Representation,
    UR: Coproduct + FromLayout<T>,
{
}

impl<T, TR, U, UR> FromLayout<Coprod![T, ...TR]> for Hlist![U, ...UR]
where
    Self: FromLayout<T>,
    T: Representation,
    U: Representation,
    TR: Coproduct,
    UR: Product,
{
}

impl<T, TR, U, UR> FromLayout<Coprod![T, ...TR]> for Hlist![U, ...UR]
where
    Self: FromLayout<TR>,
    T: Representation,
    U: Representation,
    TR: Coproduct,
    UR: Product,
{
}
/* OVERLAPPING IMPLS END HERE */


// Every variant of Coprod![T, ...TR] must be a valid instance of Self
impl<T, U, UR> FromLayout<Coprod![T]> for Hlist![U, ...UR]
where
    Self: FromLayout<T>,
    T: Representation,
    U: Representation,
    UR: Product,
{
}

assert_impl_all!(
    Coprod![Hlist![Init], Hlist![NonZero]]: FromLayout<Hlist![Init]>,
    FromLayout<Hlist![NonZero]>
);

/// Base case.
impl FromLayout<product::Nil> for product::Nil {}

assert_impl_all!(Hlist![]: FromLayout<Hlist![]>);

// U: crate::TransmuteFrom<T> indicates that the bytes of any valid T
// correspond to the bytes of a valid instance of U.

///  `Init -> *`
impl<TR, U1, UR> FromLayout<Hlist![Init, ...TR]> for Hlist![U1, ...UR]
where
    TR: Product,
    U1: Representation + FromSlot<Init>,
    UR: Product + FromLayout<TR>,
{
}

// An initialized byte may only be constructed from another initialized byte.
assert_impl_all!(Hlist![Init]: FromLayout<Hlist![Init]>);

// An initialized byte may not be constructed from an uninitialized byte.
assert_not_impl_any!(Hlist![Init]: FromLayout<Hlist![Uninit]>);

///  `NonZero -> *`
impl<TR, U1, UR> FromLayout<Hlist![NonZero, ...TR]> for Hlist![U1, ...UR]
where
    U1: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    TR: Product,
    UR: Product,
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

    U1: Representation,
    TR: Product,
    UR: Product,
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
    U1: crate::TransmuteFrom<T1>,
    UR: FromLayout<TR>,

    *const T1: Representation,
    *const U1: Representation,
    TR: Product,
    UR: Product,
{
}

//assert_impl_all!(Hlist![*const u64]: FromLayout<Hlist![*const i64]>);

//assert_not_impl_any!(Hlist![*const u64]: FromLayout<Hlist![*const i16]>);

/// `*mut T -> *const U`
impl<T1, TR, U1, UR> FromLayout<Hlist![*mut T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: crate::TransmuteFrom<T1>,
    UR: FromLayout<TR>,

    *mut T1: Representation,
    *const U1: Representation,
    TR: Product,
    UR: Product,
{
}

// A const pointer may be created from a mut pointer.
//assert_impl_all!(Hlist![*const u64]: FromLayout<Hlist![*mut u64]>);

// A mut pointer may NOT be created from a const pointer.
//assert_not_impl_any!(Hlist![*mut u64]: FromLayout<Hlist![*const u64]>);

/// `&'t mut T -> *const U`
impl<'t, T1, TR, U1, UR> FromLayout<Hlist![&'t T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: AlignedTo<T1>,
    U1: crate::TransmuteFrom<T1>,
    UR: FromLayout<TR>,

    &'t T1: Representation,
    *const U1: Representation,
    TR: Product,
    UR: Product,
{
}

// A smart pointer may be converted to a const pointer.
//assert_impl_all!(Hlist![*const u64]: FromLayout<Hlist![&'static u64]>);

// A const pointer may NOT be converter to a smart pointer.
//assert_not_impl_any!(Hlist![&'static u64]: FromLayout<Hlist![*const u64]>);

/// `&'t T -> &'u U`
impl<'t, 'u, T1, TR, U1, UR> FromLayout<Hlist![&'t T1, ...TR]> for Hlist![&'u U1, ...UR]
where
    't: 'u,
    U1: AlignedTo<T1>,
    U1: crate::TransmuteFrom<T1>,
    UR: FromLayout<TR>,

    &'t T1: Representation,
    &'u U1: Representation,

    TR: Product,
    UR: Product,
{
}

// Pointers are convertible if their underlying types are convertible.
//assert_impl_all!(Hlist![&'static u64]: FromLayout<Hlist![&'static i64]>);

// Pointers are not convertible if their underlying types aren't convertible.
//assert_not_impl_any!(Hlist![&'static u64]: FromLayout<Hlist![&'static u16]>);

/// `&mut 't T -> &'u U`
impl<'t, 'u, T1, TR, U1, UR> FromLayout<Hlist![&'t mut T1, ...TR]> for Hlist![&'u U1, ...UR]
where
    't: 'u,
    U1: AlignedTo<T1>,
    U1: crate::TransmuteFrom<T1>,
    UR: FromLayout<TR>,

    &'t mut T1: Representation,
    &'u U1: Representation,
    TR: Product,
    UR: Product,
{
}

// Pointers are convertible if their underlying types are convertible.
//assert_impl_all!(Hlist![&'static u64]: FromLayout<Hlist![&'static mut i64]>);

// Pointers are not convertible if their underlying types aren't convertible.
//assert_not_impl_any!(Hlist![&'static mut u64]: FromLayout<Hlist![&'static u16]>);

/// `&mut 't T -> *const U`
impl<'t, T1, TR, U1, UR> FromLayout<Hlist![&'t mut T1, ...TR]> for Hlist![*const U1, ...UR]
where
    U1: AlignedTo<T1>,
    U1: crate::TransmuteFrom<T1>,
    UR: FromLayout<TR>,

    &'t mut T1: Representation,
    *const U1: Representation,
    TR: Product,
    UR: Product,
{
}

// If the underlying types are convertible, a const pointer may be created from:
//assert_impl_all!(
//    Hlist![*const u64]: FromLayout<Hlist![*const i64]>,
//    FromLayout<Hlist![*mut i64]>,
//    FromLayout<Hlist![&'static i64]>,
//    FromLayout<Hlist![&'static mut i64]>
//);

// A mut smart pointer may not be created from a const pointer.
//assert_not_impl_any!(Hlist![&'static mut u64]: FromLayout<Hlist![*const u64]>);

////////////////////////////////////////////////////////////////////////////////
// Pointer Decompositions
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_pointer_width = "64")]
type InitializedBytes<R = product::Nil> =
    Hlist![Init, Init, Init, Init, Init, Init, Init, Init, ...R];

#[cfg(target_pointer_width = "64")]
type NonZeroBytes<R = product::Nil> =
    Hlist![NonZero, NonZero, NonZero, NonZero, NonZero, NonZero, NonZero, NonZero, ...R];

#[cfg(target_pointer_width = "64")]
type UninitializedBytes<R = product::Nil> =
    Hlist![Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, ...R];

// pointers may be decomposed into initialized bytes
//assert_impl_all!(
//    InitializedBytes: FromLayout<Hlist![*const u64]>,
//    FromLayout<Hlist![*mut u64]>,
//    FromLayout<Hlist![&'static u64]>,
//    FromLayout<Hlist![&'static mut u64]>
//);

// pointers may be decomposed into uninitialized bytes
//assert_impl_all!(
//    UninitializedBytes: FromLayout<Hlist![*const u64]>,
//    FromLayout<Hlist![*mut u64]>,
//    FromLayout<Hlist![&'static u64]>,
//    FromLayout<Hlist![&'static mut u64]>
//);

// smart pointers may be decomposed into nonzero bytes
//assert_impl_all!(
//    NonZeroBytes: FromLayout<Hlist![&'static u64]>,
//    FromLayout<Hlist![&'static mut u64]>
//);

// raw pointers may NOT be decomposed into nonzero bytes
//assert_not_impl_any!(
//    NonZeroBytes: FromLayout<Hlist![*const u64]>,
//    FromLayout<Hlist![*mut u64]>
//);

////////////////////////////////////////////////////////////////////////////////
// Smart Pointer Decomposition
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_pointer_width = "64")]
/// &'t T -> `[Init; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<NonZero>,
    U3: Representation + FromSlot<NonZero>,
    U4: Representation + FromSlot<NonZero>,
    U5: Representation + FromSlot<NonZero>,
    U6: Representation + FromSlot<NonZero>,
    U7: Representation + FromSlot<NonZero>,
    U8: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    &'t T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t T -> `[NonZero; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t T, ...TR]> for Hlist![NonZero, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<NonZero>,
    U3: Representation + FromSlot<NonZero>,
    U4: Representation + FromSlot<NonZero>,
    U5: Representation + FromSlot<NonZero>,
    U6: Representation + FromSlot<NonZero>,
    U7: Representation + FromSlot<NonZero>,
    U8: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    &'t T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t T -> `[Uninit; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<NonZero>,
    U3: Representation + FromSlot<NonZero>,
    U4: Representation + FromSlot<NonZero>,
    U5: Representation + FromSlot<NonZero>,
    U6: Representation + FromSlot<NonZero>,
    U7: Representation + FromSlot<NonZero>,
    U8: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    &'t T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t mut T -> `[Init; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t mut T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<NonZero>,
    U3: Representation + FromSlot<NonZero>,
    U4: Representation + FromSlot<NonZero>,
    U5: Representation + FromSlot<NonZero>,
    U6: Representation + FromSlot<NonZero>,
    U7: Representation + FromSlot<NonZero>,
    U8: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    &'t mut T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t mut T -> `[NonZero; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t mut T, ...TR]> for Hlist![NonZero, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<NonZero>,
    U3: Representation + FromSlot<NonZero>,
    U4: Representation + FromSlot<NonZero>,
    U5: Representation + FromSlot<NonZero>,
    U6: Representation + FromSlot<NonZero>,
    U7: Representation + FromSlot<NonZero>,
    U8: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    &'t mut T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// &'t mut T -> `[Uninit; target_pointer_width]`
impl<'t, T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![&'t mut T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<NonZero>,
    U3: Representation + FromSlot<NonZero>,
    U4: Representation + FromSlot<NonZero>,
    U5: Representation + FromSlot<NonZero>,
    U6: Representation + FromSlot<NonZero>,
    U7: Representation + FromSlot<NonZero>,
    U8: Representation + FromSlot<NonZero>,
    UR: FromLayout<TR>,

    &'t mut T: Representation,
    TR: Product,
    UR: Product,
{
}

////////////////////////////////////////////////////////////////////////////////
// Raw Pointer Decompositions
////////////////////////////////////////////////////////////////////////////////

#[cfg(target_pointer_width = "64")]
/// *const T -> `[Init; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*const T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<Init>,
    U3: Representation + FromSlot<Init>,
    U4: Representation + FromSlot<Init>,
    U5: Representation + FromSlot<Init>,
    U6: Representation + FromSlot<Init>,
    U7: Representation + FromSlot<Init>,
    U8: Representation + FromSlot<Init>,
    UR: FromLayout<TR>,

    *const T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// *const T -> `[Uninit; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*const T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<Init>,
    U3: Representation + FromSlot<Init>,
    U4: Representation + FromSlot<Init>,
    U5: Representation + FromSlot<Init>,
    U6: Representation + FromSlot<Init>,
    U7: Representation + FromSlot<Init>,
    U8: Representation + FromSlot<Init>,
    UR: FromLayout<TR>,

    *const T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// *mut T -> `[Init; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*mut T, ...TR]> for Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<Init>,
    U3: Representation + FromSlot<Init>,
    U4: Representation + FromSlot<Init>,
    U5: Representation + FromSlot<Init>,
    U6: Representation + FromSlot<Init>,
    U7: Representation + FromSlot<Init>,
    U8: Representation + FromSlot<Init>,
    UR: FromLayout<TR>,

    *mut T: Representation,
    TR: Product,
    UR: Product,
{
}

#[cfg(target_pointer_width = "64")]
/// *mut T -> `[Uninit; target_pointer_width]`
impl<T, TR, U2, U3, U4, U5, U6, U7, U8, UR> FromLayout<Hlist![*mut T, ...TR]> for Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
where
    U2: Representation + FromSlot<Init>,
    U3: Representation + FromSlot<Init>,
    U4: Representation + FromSlot<Init>,
    U5: Representation + FromSlot<Init>,
    U6: Representation + FromSlot<Init>,
    U7: Representation + FromSlot<Init>,
    U8: Representation + FromSlot<Init>,
    UR: FromLayout<TR>,

    *mut T: Representation,
    TR: Product,
    UR: Product,
{
}

////////////////////////////////////////////////////////////////////////////////
// Pointer Recompositions
////////////////////////////////////////////////////////////////////////////////

/// `[Init; target_pointer_width] -> *const U`
impl<TR, U1, UR> FromLayout<InitializedBytes<TR>> for Hlist![*const U1, ...UR]
where
    UR: FromLayout<TR>,

    *const U1: Representation,
    TR: Product,
    UR: Product,
{
}

// const ptr may be created from initialized bytes
//#[cfg(target_pointer_width = "64")]
//assert_impl_all!(Hlist![*const u64]: FromLayout<InitializedBytes>);

// const ptr may NOT be created from uninitialized bytes
//#[cfg(target_pointer_width = "64")]
//assert_not_impl_any!(Hlist![*const u64]: FromLayout<UninitializedBytes>);

/// `[Init; target_pointer_width] -> *mut U`
impl<TR, U1, UR> FromLayout<InitializedBytes<TR>> for Hlist![*mut U1, ...UR]
where
    UR: FromLayout<TR>,

    *mut U1: Representation,
    TR: Product,
    UR: Product,
{
}

// mut ptr may be created from initialized bytes
//#[cfg(target_pointer_width = "64")]
//assert_impl_all!(Hlist![*mut u64]: FromLayout<InitializedBytes>);

// mut ptr may NOT be created from uninitialized bytes
//#[cfg(target_pointer_width = "64")]
//assert_not_impl_any!(Hlist![*mut u64]: FromLayout<UninitializedBytes>);

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
    U: crate::TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `&'u U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U> FromSlot<&'t mut T> for &'u U
where
    U: AlignedTo<T>,
    U: crate::TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `&'u mut U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U> FromSlot<&'t mut T> for &'u mut U
where
    U: AlignedTo<T>,
    U: crate::TransmuteFrom<T>,
{
}

/// `&'t T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<'t, T, U> FromSlot<&'t T> for *const U
where
    U: AlignedTo<T>,
    U: crate::TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<'t, T, U> FromSlot<&'t mut T> for *const U
where
    U: AlignedTo<T>,
    U: crate::TransmuteFrom<T>,
{
}

/// `&'t mut T` is a valid instance of `*mut U`, if the underlying types are
/// covertible.
impl<'t, T, U> FromSlot<&'t mut T> for *mut U
where
    U: AlignedTo<T>,
    U: crate::TransmuteFrom<T>,
{
}

/// `*const T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<T, U> FromSlot<*const T> for *const U
where
    U: AlignedTo<T>,
    U: crate::TransmuteFrom<T>,
{
}
