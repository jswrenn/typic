use crate::bytelevel::{
    self as blv,
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::layout::Layout;
use crate::num::{self, UInt, UTerm};
use super::{Consume, Flatten};
use super::from_type;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait Subsumes<T> {}

/// ANYTHING -> []
unsafe impl<T> Subsumes<T> for PNil {}

mod bytes_to {
    use super::*;

    /// [Bytes|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<TKind, TSize, TRest, U, USize, URest>
    Subsumes<PCons<Bytes<TKind, TSize>, TRest>>
         for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          Subsumes<PCons<Bytes<TKind, TSize>, TRest>>
    {}

    /// [Bytes|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest>
    Subsumes<PCons<Bytes<TKind, TSize>, TRest>>
         for PCons<Bytes<UKind, USize>, URest>
    where
        Bytes<UKind, USize>: BytesSubsumes<Bytes<TKind, TSize>>,
        USize: Consume<TSize>,

        Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
        Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

        blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
          Subsumes<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>>
    {}

    /// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
    pub trait BytesSubsumes<T> {}

    macro_rules! from_bytes {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          impl<A, B, C, D>
          BytesSubsumes<Bytes<$TKind, num::UInt<A, B>>>
                    for Bytes<$UKind, num::UInt<C, D>>
          {}
        )*
      };
    }

    from_bytes![
      kind::NonZero       => kind::Uninitialized ,
      kind::NonZero       => kind::Initialized   ,
      kind::NonZero       => kind::NonZero       ,
      kind::Initialized   => kind::Uninitialized ,
      kind::Initialized   => kind::Initialized   ,
      kind::Uninitialized => kind::Uninitialized ,
    ];

    // If either sizes are empty, `BytesSubsumes` vacuously holds.
    impl<TKind, UKind> BytesSubsumes<Bytes<TKind, num::UTerm>> for Bytes<UKind, num::UTerm> {}
    impl<TKind, A, B, UKind> BytesSubsumes<Bytes<TKind, num::UInt<A, B>>> for Bytes<UKind, num::UTerm> {}
    impl<TKind, UKind, A, B> BytesSubsumes<Bytes<TKind, num::UTerm>> for Bytes<UKind, num::UInt<A, B>> {}

    /// [Bytes|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, TKind, TSize, TRest, UK, U, URest>
    Subsumes<PCons<Bytes<TKind, TSize>, TRest>>
         for PCons<Reference<'u, UK, U>, URest>
    where
        crate::TODO:,
    {}
}

mod array_to {
    use super::*;

    /// [Array|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<T, TSize, TRest, U, USize, URest>
    Subsumes<PCons<Array<T, TSize>, TRest>>
         for PCons<Array<U, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,
        PCons<Array<U, USize>, URest>: Flatten,

        <PCons<Array<U, USize>, URest> as Flatten>::Output:
            Subsumes<<PCons<Array<T, TSize>, TRest> as Flatten>::Output>,
    {}

    /// [Array|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<T, TSize, TRest, UKind, USize, URest>
    Subsumes<PCons<Array<T, TSize>, TRest>>
         for PCons<Bytes<UKind, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: Subsumes<<PCons<Array<T, TSize>, TRest> as Flatten>::Output>,
    {}

    /// [Array|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, T, TSize, TRest, UK, U, URest>
    Subsumes<PCons<Array<T, TSize>, TRest>>
         for PCons<Reference<'u, UK, U>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: Subsumes<<PCons<Array<T, TSize>, TRest> as Flatten>::Output>,
    {}
}

mod reference_to {
    use super::*;

    /// [Reference|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<'t, T, TK, TRest, U, USize, URest>
    Subsumes<PCons<Reference<'t, TK, T>, TRest>>
         for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          Subsumes<PCons<Reference<'t, TK, T>, TRest>>,
    {}

    /// [Reference|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<'t, T, TK, TRest, UKind, USize, URest>
    Subsumes<PCons<Reference<'t, TK, T>, TRest>>
         for PCons<Bytes<UKind, USize>, URest>
    where
        Self: Subsumes<ReferenceBytes<TRest>>,
    {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, T, TK, TRest, U, UK, URest>
    Subsumes<PCons<Reference<'t, TK, T>, TRest>>
         for PCons<Reference<'u, UK, U>, URest>
    where
        crate::TODO:,
    {}
}

#[cfg(test)] const _ : () = {
  use static_assertions::*;
  use crate::num::*;

  assert_impl_all!(PNil: Subsumes<PNil>);

  assert_impl_all!(PCons<PaddingSlot<U0>, PNil>: Subsumes<PCons<PaddingSlot<U0>, PNil>>);
};