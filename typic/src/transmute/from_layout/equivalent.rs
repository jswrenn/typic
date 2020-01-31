use crate::bytelevel::{
    self as blv,
    slot::*,
    PCons, PNil, ReferenceBytes,
};
use super::from_type;
use super::Consume;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait Equivalent<T> {}

/// ANYTHING -> []
unsafe impl<T> Equivalent<T> for PNil {}

/// [Bytes|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest>
Equivalent<PCons<Bytes<TKind, TSize>, TRest>>
     for PCons<Bytes<UKind, USize>, URest>
where
    USize: Consume<TSize>,

    Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
    Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

    blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
      Equivalent<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>>
{}

/// [SharedRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
Equivalent<PCons<SharedRef<'t, T>, TRest>>
       for PCons<SharedRef<'u, U>, URest>
where
    't: 'u,
    U: from_type::Equivalent<T>,
{}

/// [UniqueRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
Equivalent<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<SharedRef<'u, U>, URest>
where
    't: 'u,
    U: from_type::Equivalent<T>,
{}

/// [UniqueRef|_] -> [UniqueRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
Equivalent<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<UniqueRef<'u, U>, URest>
where
    't: 'u,
    U: from_type::Equivalent<T>,
{}

/// [UniqueRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
Equivalent<PCons<UniqueRef<'t, T>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    Self: Equivalent<ReferenceBytes<TRest>>
{}

/// [SharedRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
Equivalent<PCons<SharedRef<'t, T>, TRest>>
       for PCons<Bytes<UKind, USize>, URest>
where
    Self: Equivalent<ReferenceBytes<TRest>>
{}
