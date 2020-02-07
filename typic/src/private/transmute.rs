use crate::private::highlevel::Transparent;
use core::mem;

pub struct Relax;
pub struct Constrain;

pub struct Safe;
pub struct Sound;

#[rustfmt::skip]
mod from_type;

#[rustfmt::skip]
mod from_layout;

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
pub unsafe trait TransmuteInto<U>: Sized {
    /// Reinterprets the bits of `self` as type `U`.
    fn transmute_into(self) -> U;
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
pub unsafe trait TransmuteFrom<T>: Sized {
    /// Reinterprets the bits of `from` as type `Self`.
    fn transmute_from(from: T) -> Self;
}

unsafe impl<T, U> TransmuteInto<U> for T
where
    U: TransmuteFrom<T>,
{
    #[inline(always)]
    fn transmute_into(self) -> U {
        U::transmute_from(self)
    }
}

unsafe impl<T, U> TransmuteFrom<T> for U
where
    U: Transparent + from_type::FromType<T, Relax, Safe>,
{
    #[inline(always)]
    fn transmute_from(from: T) -> U {
        unsafe { transmute_safe(from) }
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
pub fn transmute_safe<T, U>(from: T) -> U
where
    U: Transparent + from_type::FromType<T, Relax, Safe>,
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
pub unsafe fn transmute_sound<T, U>(from: T) -> U
where
    U: from_type::FromType<T, Relax, Sound>,
{
    let to = mem::transmute_copy(&from);
    mem::forget(from);
    to
}
