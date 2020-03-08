use core::mem;

/// Allow bit-validity to expand.
pub struct Variant;
/// Constrain bit-validity to be equal.
pub struct Invariant;

/// Alignment of pointers is statically checked.
pub struct Static;
/// Alignment of pointers is unchecked.
pub struct Unchecked;

/// Transparency is enforced.
pub struct Enforced;
/// Transparency is not enforced.
pub struct Unenforced;

pub mod neglect;

#[rustfmt::skip]
pub mod from_type;

#[rustfmt::skip]
pub mod from_layout;

/// The source and destination types **must** have
/// [stable ABIs][crate::marker::StableABI].
pub struct Stable;

/// The source and destination types **may not** both have
/// [stable ABIs][crate::marker::StableABI].
///
/// A transmutation between types with unstable ABIs is not necessarily
/// unsafe, but the creators of the source and destination types do **not**
/// guarantee that those types will have the same size, alignment, or
/// data arrangement across minor releases.
pub struct Unstable;

/// A ***safe*** and ***sound*** value-to-value conversion.
/// The opposite of [`TransmuteFrom`].
///
/// `TransmuteInto<U>` is only implemented for `T` when
/// 1. [`T` is ***soundly*** transmutable into `U`][soundness], and
/// 2. [`T` is ***safely*** transmutable into `U`][safety].
///
/// See also [`transmute_safe`].
///
/// [`TransmuteFrom`]: crate::TransmuteFrom
/// [`transmute_safe`]: crate::transmute_safe
/// [soundness]: crate::sound#when-is-a-transmutation-sound
/// [safety]: crate::safe
pub unsafe trait TransmuteInto<U, O = ()>: Sized
where
    O: neglect::Options,
{
    /// Reinterprets the bits of `self` as type `U`.
    fn transmute_into(self) -> U;
}

unsafe impl<T, U, O> TransmuteInto<U, O> for T
where
    U: TransmuteFrom<T, O>,
    O: neglect::Options,
{
    #[inline(always)]
    fn transmute_into(self) -> U {
        U::transmute_from(self)
    }
}

/// For ergonomics, until [rust-lang/rust#27336] is resolved.
///
/// [rust-lang/rust#27336]: https://github.com/rust-lang/rust/issues/27336
pub trait StableTransmuteInto<U>: TransmuteInto<U> {
    fn transmute_into(self) -> U;
}

impl<T, U> StableTransmuteInto<U> for T
where
    T: TransmuteInto<U>,
{
    #[inline(always)]
    fn transmute_into(self) -> U
    {
        self.transmute_into()
    }
}

/// A ***safe*** and ***sound*** value-to-value conversion.
/// The opposite of [`TransmuteInto`].
///
/// `TransmuteFrom<T>` is only implemented for `U` when
/// 1. [`T` is ***soundly*** transmutable into `U`][soundness], and
/// 2. [`T` is ***safely*** transmutable into `U`][safety].
///
/// See also [`transmute_safe`].
///
/// [`TransmuteInto`]: crate::TransmuteInto
/// [`transmute_safe`]: crate::transmute_safe
/// [soundness]: crate::sound#when-is-a-transmutation-sound
/// [safety]: crate::safe
pub unsafe trait TransmuteFrom<T, O = ()>: Sized
where
    O: neglect::Options,
{
    /// Reinterprets the bits of `from` as type `Self`.
    fn transmute_from(from: T) -> Self;
}

unsafe impl<T, U, O> TransmuteFrom<T, O> for U
where
    U: UnsafeTransmuteFrom<T, O>,
    O: neglect::Options,
{
    #[inline(always)]
    fn transmute_from(from: T) -> U {
        unsafe { U::unsafe_transmute_from(from) }
    }
}


pub unsafe trait UnsafeTransmuteFrom<T, O = ()>: Sized
where
    O: neglect::UnsafeOptions,
{
    /// Reinterprets the bits of `from` as type `Self`.
    unsafe fn unsafe_transmute_from(from: T) -> Self;
}

unsafe impl<T, U, O> UnsafeTransmuteFrom<T, O> for U
where
    U: from_type::FromType<T,
        Variant,
        <O as neglect::UnsafeOptions>::Alignment,
        <O as neglect::UnsafeOptions>::Transparency,
        <O as neglect::UnsafeOptions>::Stability,
      >,
    O: neglect::Options,
{
    #[inline(always)]
    unsafe fn unsafe_transmute_from(from: T) -> U {
        unsafe { transmute_safe::<T, U, O>(from) }
    }
}

/// A ***safe*** and ***sound*** value-to-value conversion.
///
/// Consumes a value of type `T` and produces a value of type `U` by
/// reinterpreting that value's bits.
///
/// This will only convert `T` into `U` when:
/// 1. [`T` is ***soundly*** transmutable into `U`][soundness], and
/// 2. [`T` is ***safely*** transmutable into `U`][safety].
///
/// See also [`TransmuteInto`] and [`TransmuteFrom`].
///
/// [`TransmuteFrom`]: crate::TransmuteFrom
/// [`TransmuteInto`]: crate::TransmuteInto
/// [soundness]: crate::sound#when-is-a-transmutation-sound
/// [safety]: crate::safe
#[inline(always)]
pub fn transmute_safe<T, U, O>(from: T) -> U
where
    U: from_type::FromType<T,
        Variant,
        <O as neglect::UnsafeOptions>::Alignment,
        <O as neglect::UnsafeOptions>::Transparency,
        <O as neglect::UnsafeOptions>::Stability,
      >,
    O: neglect::Options,
{
    unsafe {
        let to = mem::transmute_copy(&from);
        mem::forget(from);
        to
    }
}

/// A ***sound*** value-to-value conversion.
///
/// Consumes a value of type `T` and produces a value of type `U` by
/// reinterpreting that value's bits.
///
/// This will only convert `T` into `U` when:
/// 1. [`T` is ***soundly*** transmutable into `U`][soundness].
///
/// It is **unsafe**, because `U` may be a user-defined type that enforces
/// additional validity restrictions in its constructor(s). This function
/// bypasses those restrictions, and may lead to later unsoundness.
///
/// [soundness]: crate::sound#when-is-a-transmutation-sound
/// [safety]: crate::safe
#[inline(always)]
pub unsafe fn transmute_sound<T, U, O>(from: T) -> U
where
    U: from_type::FromType<T,
        Variant,
        <O as neglect::UnsafeOptions>::Alignment,
        <O as neglect::UnsafeOptions>::Transparency,
        <O as neglect::UnsafeOptions>::Stability>,
    O: neglect::UnsafeOptions,
{
    let to = mem::transmute_copy(&from);
    mem::forget(from);
    to
}
