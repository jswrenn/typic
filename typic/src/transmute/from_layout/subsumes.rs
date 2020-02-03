use crate::bytelevel::{
    self as blv,
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use super::Consume;
use super::from_type;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait Subsumes<T> {}

/// ANYTHING -> []
unsafe impl<T> Subsumes<T> for PNil {}

/// [Bytes|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest>
Subsumes<PCons<Bytes<TKind, TSize>, TRest>>
     for PCons<Bytes<UKind, USize>, URest>
where
    UKind: FromKind<TKind>,
    USize: Consume<TSize>,

    Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
    Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

    blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
      Subsumes<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>>
{}

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
Subsumes<PCons<SharedRef<'t, T>, TRest>>
     for PCons<SharedRef<'u, U>, URest>
where
    't: 'u,
    U: from_type::Equivalent<T>,
{}

/// [UniqueRef|_] -> [SharedRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
Subsumes<PCons<UniqueRef<'t, T>, TRest>>
     for PCons<SharedRef<'u, U>, URest>
where
    't: 'u,
    U: from_type::Equivalent<T>,
{}

/// [UniqueRef|_] -> [UniqueRef|_]
#[rustfmt::skip] unsafe impl<'t, 'u, T, TRest, U, URest>
Subsumes<PCons<UniqueRef<'t, T>, TRest>>
     for PCons<UniqueRef<'u, U>, URest>
where
    't: 'u,
    U: from_type::Equivalent<T>,
{}

/// [UniqueRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
Subsumes<PCons<UniqueRef<'t, T>, TRest>>
     for PCons<Bytes<UKind, USize>, URest>
where
    Self: Subsumes<ReferenceBytes<TRest>>
{}

/// [SharedRef|_] -> [Bytes|_]
#[rustfmt::skip] unsafe impl<'t, T, TRest, UKind, USize, URest>
Subsumes<PCons<SharedRef<'t, T>, TRest>>
     for PCons<Bytes<UKind, USize>, URest>
where
    Self: Subsumes<ReferenceBytes<TRest>>
{}

#[cfg(test)] const _ : () = {
  use static_assertions::*;
  use crate::num::*;

  assert_impl_all!(PNil: Subsumes<PNil>);

  assert_impl_all!(PCons<PaddingSlot<U0>, PNil>: Subsumes<PCons<PaddingSlot<U0>, PNil>>);
};