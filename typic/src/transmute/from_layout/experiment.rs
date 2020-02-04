use crate::bytelevel::{
    self as blv,
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::layout::{Layout, AlignedTo};
use crate::num::{self, UInt, UTerm};
use super::{Consume, Flatten};
use super::from_type;

pub struct Constrain;
pub struct Relax;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<T, M> {}

/// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
pub trait BytesFromLayout<T, M> {}

macro_rules! from_bytes {
  ($($TKind: path => $UKind: path,)*) => {
    $(
      impl<O, A, B, C, D>
      BytesFromLayout<Bytes<$TKind, num::UInt<A, B>>, O>
                  for Bytes<$UKind, num::UInt<C, D>>
      {}
    )*
  };
}

from_bytes![
  kind::NonZero       => kind::NonZero       ,
  kind::Initialized   => kind::Initialized   ,
  kind::Uninitialized => kind::Uninitialized ,
];

// If either sizes are empty, `BytesFromLayout` vacuously holds.
impl<TKind, UKind> BytesFromLayout<Bytes<TKind, num::UTerm>, Relax> for Bytes<UKind, num::UTerm> {}
impl<TKind, A, B, UKind> BytesFromLayout<Bytes<TKind, num::UInt<A, B>>, Relax> for Bytes<UKind, num::UTerm> {}
impl<TKind, UKind, A, B> BytesFromLayout<Bytes<TKind, num::UTerm>, Relax> for Bytes<UKind, num::UInt<A, B>> {}

/// ANYTHING -> []
unsafe impl<T, O> FromLayout<T, O> for PNil {}

mod bytes_to {
    use super::*;

    /// [Bytes|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<O, TKind, TSize, TRest, U, USize, URest>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, O>
              for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Bytes<TKind, TSize>, TRest>, O>
    {}

    /// [Bytes|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<O, TKind, TSize, TRest, UKind, USize, URest>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, O>
              for PCons<Bytes<UKind, USize>, URest>
    where
        Bytes<UKind, USize>: BytesFromLayout<Bytes<TKind, TSize>, O>,
        USize: Consume<TSize>,

        Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
        Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

        blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
          FromLayout<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>, O>
    {}

    /// [Bytes|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, O, TKind, TSize, TRest, UK, U, URest>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, O>
              for PCons<Reference<'u, UK, U>, URest>
    where
        crate::TODO:,
    {}
}

mod array_to {
    use super::*;

    /// [Array|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<O, T, TSize, TRest, U, USize, URest>
    FromLayout<PCons<Array<T, TSize>, TRest>, O>
              for PCons<Array<U, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,
        PCons<Array<U, USize>, URest>: Flatten,

        <PCons<Array<U, USize>, URest> as Flatten>::Output:
            FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, O>,
    {}

    /// [Array|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<O, T, TSize, TRest, UKind, USize, URest>
    FromLayout<PCons<Array<T, TSize>, TRest>, O>
              for PCons<Bytes<UKind, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, O>,
    {}

    /// [Array|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, O, T, TSize, TRest, UK, U, URest>
    FromLayout<PCons<Array<T, TSize>, TRest>, O>
              for PCons<Reference<'u, UK, U>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, O>,
    {}
}

mod reference_to {
    use super::*;

    /// [Reference|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<'t, O, T, TK, TRest, U, USize, URest>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, O>
              for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Reference<'t, TK, T>, TRest>, O>,
    {}

    /// [Reference|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<'t, O, T, TK, TRest, UKind, USize, URest>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, O>
              for PCons<Bytes<UKind, USize>, URest>
    where
        Self: FromLayout<ReferenceBytes<TRest>, O>,
    {}

    pub trait ReferenceInto<T> {}

    impl ReferenceInto<Shared> for Shared {}
    impl ReferenceInto<Shared> for Unique {}
    impl ReferenceInto<Unique> for Unique {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, O, T, TK, TRest, U, UK, URest>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, O>
              for PCons<Reference<'u, UK, U>, URest>
    where
        't: 'u,
        U: AlignedTo<T>,
        UK: ReferenceInto<TK>,
        U: from_type::FromType<Constrain, T>,
    {}
}

#[cfg(test)] const _ : () = {
  use static_assertions::*;
  use crate::num::*;

  assert_impl_all!(PNil: FromLayout<PNil>);

  assert_impl_all!(PCons<PaddingSlot<U0>, PNil>: FromLayout<PCons<PaddingSlot<U0>, PNil>>);
};