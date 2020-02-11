use crate::private::bytelevel::{
    self as blv,
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::private::highlevel::Transparent;
use crate::private::layout::{Layout, AlignedTo};
use crate::private::num::{self, UInt, UTerm};
use super::from_type::FromType;
use super::{Relax, Constrain, Safe, Sound};

mod consume;
pub use consume::Consume;

mod flatten;
pub use flatten::Flatten;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<T, M, S> {}

/// ANYTHING -> []
unsafe impl<T, M, S> FromLayout<T, M, S> for PNil {}

#[rustfmt::skip] unsafe impl<UKind, URest, M, S>
FromLayout<PNil, M, S>
       for PCons<Bytes<UKind, UTerm>, URest>
where
    URest: FromLayout<PNil, M, S>,
{}

#[rustfmt::skip] unsafe impl<U, URest, M, S>
FromLayout<PNil, M, S>
       for PCons<Array<U, UTerm>, URest>
where
    URest: FromLayout<PNil, M, S>,
{}

mod bytes_to {
    use super::*;

    /// [Bytes|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<TKind, TSize, TRest, U, USize, URest, M, S>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, M, S>
         for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Bytes<TKind, TSize>, TRest>, M, S>
    {}

    /// [Bytes|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest, M, S>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, M, S>
         for PCons<Bytes<UKind, USize>, URest>
    where
        Bytes<UKind, USize>: BytesFromBytes<Bytes<TKind, TSize>, M, S>,
        USize: Consume<TSize>,

        Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
        Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

        blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
          FromLayout<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>, M, S>
    {}

    /// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
    pub trait BytesFromBytes<T, M, S> {}

    macro_rules! constrain {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          impl<A, B, C, D, M, S>
          BytesFromBytes<Bytes<$TKind, num::UInt<A, B>>, M, S>
                    for Bytes<$UKind, num::UInt<C, D>>
          {}
        )*
      };
    }

    constrain![
      kind::NonZero       => kind::NonZero       ,
      kind::Initialized   => kind::Initialized   ,
      kind::Uninitialized => kind::Uninitialized ,
    ];

    macro_rules! relax {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          impl<A, B, C, D, S>
          BytesFromBytes<Bytes<$TKind, num::UInt<A, B>>, Relax, S>
                    for Bytes<$UKind, num::UInt<C, D>>
          {}
        )*
      };
    }

    relax![
      kind::NonZero       => kind::Uninitialized ,
      kind::NonZero       => kind::Initialized   ,
      kind::Initialized   => kind::Uninitialized ,
    ];


    // If either sizes are empty, `BytesFromBytes` vacuously holds.
    impl<TKind, UKind, M, S> BytesFromBytes<Bytes<TKind, num::UTerm>, M, S> for Bytes<UKind, num::UTerm> {}
    impl<TKind, A, B, UKind, M, S> BytesFromBytes<Bytes<TKind, num::UInt<A, B>>, M, S> for Bytes<UKind, num::UTerm> {}
    impl<TKind, UKind, A, B, M, S> BytesFromBytes<Bytes<TKind, num::UTerm>, M, S> for Bytes<UKind, num::UInt<A, B>> {}

    /// [Bytes|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, TKind, TRest, UK, U, URest, M, S>
    FromLayout<PCons<Bytes<TKind, num::UTerm>, TRest>, M, S>
         for PCons<Reference<'u, UK, U>, URest>
    where
        Self: FromLayout<TRest, M, S>,
    {}
}

mod array_to {
    use super::*;

    /// [Array|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<T, TSize, TRest, U, USize, URest, M, S>
    FromLayout<PCons<Array<T, TSize>, TRest>, M, S>
         for PCons<Array<U, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,
        PCons<Array<U, USize>, URest>: Flatten,

        <PCons<Array<U, USize>, URest> as Flatten>::Output:
            FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, M, S>,
    {}

    /// [Array|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<T, TSize, TRest, UKind, USize, URest, M, S>
    FromLayout<PCons<Array<T, TSize>, TRest>, M, S>
         for PCons<Bytes<UKind, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, M, S>,
    {}

    /// [Array|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, T, TSize, TRest, UK, U, URest, M, S>
    FromLayout<PCons<Array<T, TSize>, TRest>, M, S>
         for PCons<Reference<'u, UK, U>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, M, S>,
    {}
}

mod reference_to {
    use super::*;

    /// [Reference|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<'t, T, TK, TRest, U, USize, URest, M, S>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M, S>
         for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Reference<'t, TK, T>, TRest>, M, S>,
    {}

    /// [Reference|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<'t, T, TK, TRest, UKind, USize, URest, M, S>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M, S>
         for PCons<Bytes<UKind, USize>, URest>
    where
        Self: FromLayout<ReferenceBytes<TRest>, M, S>,
    {}

    pub trait FromMutability<T> {}
    impl FromMutability<Unique> for Unique {}
    impl FromMutability<Unique> for Shared {}
    impl FromMutability<Shared> for Shared {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, T, TK, TRest, U, UK, URest, M>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M, Safe>
         for PCons<Reference<'u, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        T: Transparent,
        U: Transparent + AlignedTo<T> + FromType<T, Constrain, Safe>,
    {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, T, TK, TRest, U, UK, URest, M>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M, Sound>
         for PCons<Reference<'u, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        U: Transparent + AlignedTo<T> + FromType<T, Constrain, Sound>,
    {}
}


#[cfg(test)]
mod test {
  use super::*;

  fn subsumes<T, U: FromLayout<T, Relax, Sound>>()
  {}

  macro_rules! P {
    () => { crate::private::bytelevel::PNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { P![$A,] };
    ($A:ty, $($tok:tt)*) => {
        crate::private::bytelevel::PCons<$A, P![$($tok)*]>
    };
  }

  #[test]
  fn test() {
    use crate::private::{self, num::*, highlevel::Type, layout::Layout};
    use crate::private::bytelevel::slot::{bytes::kind, *};
    use static_assertions::*;
    use crate::private::bytelevel as blvl;

    subsumes::<
      P![PaddingSlot<U2>],
      P![]
    >();

    subsumes::<
      P![PaddingSlot<U2>],
      P![PaddingSlot<U1>]
    >();

    subsumes::<
      P![PaddingSlot<U1>, PaddingSlot<U1>],
      P![PaddingSlot<U2>]
    >();
  }
}
