use crate::highlevel::Transparent;
use core::mem;

pub struct Relax;
pub struct Constrain;

#[rustfmt::skip]
mod from_type;

#[rustfmt::skip]
mod from_layout;

/// A value-to-value conversion that consumes the input value. The
/// opposite of [`TransmuteFrom`].
///
/// Implemented only for types where all possibile instantiations of `Self` are
/// also valid instantiations of `T`.
///
/// Prefer using [`TransmuteInto`] over [`TransmuteFrom`] when specifying trait
/// bounds on a generic function to ensure that types that only implement
/// [`TransmuteInto`] can be used as well.
pub unsafe trait TransmuteInto<T>: Sized {
    /// Performs the conversion.
    fn transmute_into(self) -> T;
}

/// Used to do value-to-value conversions while consuming the input value. It is
/// the reciprocal of [`TransmuteInto`].
pub unsafe trait TransmuteFrom<T>: Sized {
    /// Performs the conversion.
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
    U: from_type::FromType<T, Relax>,
{
    #[inline(always)]
    fn transmute_from(from: T) -> U {
        unsafe { transmute(from) }
    }
}

/// Reinterprets the bits of a value of one type as another type.
///
/// This function is only callable for instances in which all possible
/// instantiations of `T` are also bit-valid instances of `U`.
///
/// It is **unsafe**, because `U` may be a user-defined type that enforces
/// additional validity restrictions in its constructor(s). This function
/// bypasses those restrictions, and may lead to later unsoundness.
pub unsafe fn transmute<T, U>(from: T) -> U
where
    U: from_type::FromType<T, Relax>,
{
    let to = mem::transmute_copy(&from);
    mem::forget(from);
    to
}
