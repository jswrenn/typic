use crate::bytelevel::{
    self as blv,
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::num;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<T> {}

/// ANYTHING -> []
unsafe impl<T> FromLayout<T> for PNil {}

/// [Bytes|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest>
FromLayout<PCons<Bytes<TKind, TSize>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    UKind: FromKind<TKind>,
    USize: Consume<TSize>,

    Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
    Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

    blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
      FromLayout<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>>
{}

/// Consume `Maximum<TSize, USize>` of the leading bytes of two layouts.
pub trait Consume<TSize> {
    /// The number of bytes to append back on `TRest`.
    type TSize;

    /// The number of bytes to append back on `URest`.
    type USize;
}

#[rustfmt::skip]
impl<TSize, USize> Consume<TSize> for USize
where
    TSize: num::Max<USize>,
    TSize: num::Sub<num::Maximum<TSize, USize>>,
    USize: num::Sub<num::Maximum<TSize, USize>>,
{
    type TSize = num::Diff<TSize, num::Maximum<TSize, USize>>;
    type USize = num::Diff<USize, num::Maximum<TSize, USize>>;
}

/// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
pub trait FromKind<TKind> {}
#[rustfmt::skip] impl FromKind<kind::NonZero       > for kind::Uninitialized {}
#[rustfmt::skip] impl FromKind<kind::NonZero       > for kind::Initialized   {}
#[rustfmt::skip] impl FromKind<kind::NonZero       > for kind::NonZero       {}
#[rustfmt::skip] impl FromKind<kind::Initialized   > for kind::Uninitialized {}
#[rustfmt::skip] impl FromKind<kind::Initialized   > for kind::Initialized   {}
#[rustfmt::skip] impl FromKind<kind::Uninitialized > for kind::Uninitialized {}

/// [SharedRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
FromLayout<PCons<SharedRef<'t, T>, TRest>>
       for PCons<SharedRef<'u, U>, URest>
where
    crate::TODO:
{}

/// [UniqueRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
FromLayout<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<SharedRef<'u, U>, URest>
where
    crate::TODO:
{}

/// [UniqueRef|_] -> [UniqueRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
FromLayout<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<UniqueRef<'u, U>, URest>
where
    crate::TODO:
{}

/// [UniqueRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
FromLayout<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    Self: FromLayout<ReferenceBytes<TRest>>
{}

/// [SharedRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
FromLayout<PCons<SharedRef<'t, T>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    Self: FromLayout<ReferenceBytes<TRest>>
{}
