use crate::bytelevel::{
    self as blv,
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::layout::{Layout, AlignedTo};
use crate::num::{self, UInt, UTerm};
use super::from_type::FromType;
use super::{Relax, Constrain};

mod consume;
pub use consume::Consume;

mod flatten;
pub use flatten::Flatten;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<T, M> {}

/// ANYTHING -> []
unsafe impl<T, M> FromLayout<T, M> for PNil {}

mod bytes_to {
    use super::*;

    /// [Bytes|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<TKind, TSize, TRest, U, USize, URest, M>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, M>
         for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Bytes<TKind, TSize>, TRest>, M>
    {}

    /// [Bytes|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TKind, TSize, TRest, UKind, USize, URest, M>
    FromLayout<PCons<Bytes<TKind, TSize>, TRest>, M>
         for PCons<Bytes<UKind, USize>, URest>
    where
        Bytes<UKind, USize>: BytesFromBytes<Bytes<TKind, TSize>, M>,
        USize: Consume<TSize>,

        Bytes<TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
        Bytes<UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

        blv::Sum<Bytes<UKind, <USize as Consume<TSize>>::USize>, URest>:
          FromLayout<blv::Sum<Bytes<TKind, <USize as Consume<TSize>>::TSize>, TRest>, M>
    {}

    /// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
    pub trait BytesFromBytes<T, M> {}

    /*
    macro_rules! from_bytes {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          impl<A, B, C, D, M>
          BytesFromBytes<Bytes<$TKind, num::UInt<A, B>>, M>
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
    */
    macro_rules! constrain {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          impl<A, B, C, D, M>
          BytesFromBytes<Bytes<$TKind, num::UInt<A, B>>, M>
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
          impl<A, B, C, D>
          BytesFromBytes<Bytes<$TKind, num::UInt<A, B>>, Relax>
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
    impl<TKind, UKind, M> BytesFromBytes<Bytes<TKind, num::UTerm>, M> for Bytes<UKind, num::UTerm> {}
    impl<TKind, A, B, UKind, M> BytesFromBytes<Bytes<TKind, num::UInt<A, B>>, M> for Bytes<UKind, num::UTerm> {}
    impl<TKind, UKind, A, B, M> BytesFromBytes<Bytes<TKind, num::UTerm>, M> for Bytes<UKind, num::UInt<A, B>> {}

    /// [Bytes|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, TKind, TRest, UK, U, URest, M>
    FromLayout<PCons<Bytes<TKind, num::UTerm>, TRest>, M>
         for PCons<Reference<'u, UK, U>, URest>
    where
        Self: FromLayout<TRest, M>,
    {}
}

mod array_to {
    use super::*;

    /// [Array|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<T, TSize, TRest, U, USize, URest, M>
    FromLayout<PCons<Array<T, TSize>, TRest>, M>
         for PCons<Array<U, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,
        PCons<Array<U, USize>, URest>: Flatten,

        <PCons<Array<U, USize>, URest> as Flatten>::Output:
            FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, M>,
    {}

    /// [Array|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<T, TSize, TRest, UKind, USize, URest, M>
    FromLayout<PCons<Array<T, TSize>, TRest>, M>
         for PCons<Bytes<UKind, USize>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, M>,
    {}

    /// [Array|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, T, TSize, TRest, UK, U, URest, M>
    FromLayout<PCons<Array<T, TSize>, TRest>, M>
         for PCons<Reference<'u, UK, U>, URest>
    where
        PCons<Array<T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<T, TSize>, TRest> as Flatten>::Output, M>,
    {}
}

mod reference_to {
    use super::*;

    /// [Reference|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<'t, T, TK, TRest, U, USize, URest, M>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M>
         for PCons<Array<U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Reference<'t, TK, T>, TRest>, M>,
    {}

    /// [Reference|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<'t, T, TK, TRest, UKind, USize, URest, M>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M>
         for PCons<Bytes<UKind, USize>, URest>
    where
        Self: FromLayout<ReferenceBytes<TRest>, M>,
    {}

    pub trait FromMutability<T> {}
    impl FromMutability<Unique> for Unique {}
    impl FromMutability<Unique> for Shared {}
    impl FromMutability<Shared> for Shared {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, T, TK, TRest, U, UK, URest, M>
    FromLayout<PCons<Reference<'t, TK, T>, TRest>, M>
         for PCons<Reference<'u, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        U: AlignedTo<T> + FromType<T, Constrain>,
    {}
}


#[cfg(test)]
mod test {
  use super::*;

  fn subsumes<T, U: FromLayout<T, Relax>>()
  {}

  macro_rules! P {
    () => { crate::bytelevel::PNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { P![$A,] };
    ($A:ty, $($tok:tt)*) => {
        crate::bytelevel::PCons<$A, P![$($tok)*]>
    };
  }

  #[test]
  fn test() {
    use crate::typic::{self, num::*, highlevel::Type, layout::Layout};
    use crate::typic::bytelevel::slot::{bytes::kind, *};
    use static_assertions::*;
    use crate::bytelevel as blvl;

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
