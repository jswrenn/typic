use crate::private::bytelevel::{
    self as blv,
    slot::{Pub, Priv},
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::private::layout::{Layout, AlignedTo};
use crate::private::num::{self, UInt, UTerm};
use super::from_type::FromType;
use super::{Variant, Invariant, Static, Unchecked, Enforced, Unenforced, Stable, Unstable};
use crate::stability::*;
mod consume;
pub use consume::Consume;

mod flatten;
pub use flatten::Flatten;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<
  SourceLayout,
  // Can bit-validity be widened?
  Variance,
  // Is alignment checked?
  Alignment,
  // Is library safety checked?
  Transparency,
  // Must the source and destination types have recursively stable ABIs?
  Stability,
> {}

/// ANYTHING -> []
unsafe impl<
  SourceLayout,
  Variance,
  Alignment,
  Transparency,
  Stability,
>
FromLayout<
  SourceLayout,
  Variance,
  Alignment,
  Transparency,
  Stability,
> for PNil {}

#[rustfmt::skip] unsafe impl<
  UVis, UKind, URest,
  Variance,
  Alignment,
  Transparency,
  Stability,
>
FromLayout<PNil,
  Variance,
  Alignment,
  Transparency,
  Stability,
> for PCons<Bytes<UVis, UKind, UTerm>, URest>
where
    URest: FromLayout<PNil, Variance, Alignment, Transparency, Stability>,
{}

#[rustfmt::skip] unsafe impl<
  UVis, U, URest,
  Variance,
  Alignment,
  Transparency,
  Stability,
> FromLayout<PNil, Variance, Alignment, Transparency, Stability>
         for PCons<Array<UVis, U, UTerm>, URest>
where
    URest: FromLayout<PNil, Variance, Alignment, Transparency, Stability>,
{}


/// ```rust
/// use typic::private::transmute::{Stable, Variant, Static, Enforced, from_layout::FromLayout};
/// use typic::private::bytelevel::{PCons, PNil, slot::{InitializedSlot, Pub, Priv}};
/// use typic::private::num::U1;
/// fn can_transmute<T, U: FromLayout<T, Variance, Static, Transparency, Stable>, Variance, Transparency>() {}
///
/// can_transmute::<
///   PCons<InitializedSlot<Pub, U1>, PNil>,
///   PCons<InitializedSlot<Pub, U1>, PNil>,
///   Variant, Enforced>();
///
/// can_transmute::<
///   PCons<InitializedSlot<Priv, U1>, PNil>,
///   PCons<InitializedSlot<Pub, U1>, PNil>,
///   Variant, Enforced>();
/// ```
mod bytes_to {
    use super::*;

    /// [Bytes|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<TVis, TKind, TSize, TRest, UVis, U, USize, URest,
        Variance, Alignment, Transparency, Stability,
    >
    FromLayout<PCons<Bytes<TVis, TKind, TSize>, TRest>, Variance, Alignment, Transparency, Stability>
           for PCons<Array<UVis, U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Bytes<TVis, TKind, TSize>, TRest>, Variance, Alignment, Transparency, Stability>
    {}

    /// [Bytes|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TVis, TKind, TSize, TRest, UVis, UKind, USize, URest,
      Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Bytes<TVis, TKind, TSize>, TRest>, Variance, Alignment, Transparency, Stability>
           for PCons<Bytes<UVis, UKind, USize>, URest>
    where
        Bytes<UVis, UKind, USize>: BytesFromBytes<Bytes<TVis, TKind, TSize>, Variance, Transparency>,
        USize: Consume<TSize>,

        Bytes<TVis, TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
        Bytes<UVis, UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

        blv::Sum<Bytes<UVis, UKind, <USize as Consume<TSize>>::USize>, URest>:
          FromLayout<blv::Sum<Bytes<TVis, TKind, <USize as Consume<TSize>>::TSize>, TRest>, Variance, Alignment, Transparency, Stability>
    {}

    /// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
    pub trait BytesFromBytes<T, Variance, Transparency> {}

    macro_rules! constrain {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          /// Regardless of variance and transparency, this `pub` to `pub` conversion is safe.
          impl<A, B, C, D, Variance, Transparency>
          BytesFromBytes<Bytes<Pub,  $TKind, num::UInt<A, B>>, Variance, Transparency>
                     for Bytes<Pub,  $UKind, num::UInt<C, D>>
          {}

          /// A `priv` to `pub` conversion is safe only if the transmutation is variant.
          impl<A, B, C, D, Transparency>
          BytesFromBytes<Bytes<Priv, $TKind, num::UInt<A, B>>, Variant, Transparency>
                     for Bytes<Pub,  $UKind, num::UInt<C, D>>
          {}

          /// A `priv`/`pub` to `priv` conversion is only safe if transparency is unchecked.
          impl<A, B, C, D, TVis, Variance>
          BytesFromBytes<Bytes<TVis, $TKind, num::UInt<A, B>>, Variance, Unenforced>
                     for Bytes<Priv, $UKind, num::UInt<C, D>>
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
          /// Regardless of variance and transparency, this `pub` to `pub` conversion is safe.
          impl<A, B, C, D, Transparency>
          BytesFromBytes<Bytes<Pub,  $TKind, num::UInt<A, B>>, Variant, Transparency>
                     for Bytes<Pub,  $UKind, num::UInt<C, D>>
          {}

          /// A `priv` to `pub` conversion is safe only if the transmutation is variant.
          impl<A, B, C, D, Transparency>
          BytesFromBytes<Bytes<Priv, $TKind, num::UInt<A, B>>, Variant, Transparency>
                     for Bytes<Pub,  $UKind, num::UInt<C, D>>
          {}

          /// A `priv`/`pub` to `priv` conversion is only safe if transparency is unchecked.
          impl<A, B, C, D, TVis>
          BytesFromBytes<Bytes<TVis, $TKind, num::UInt<A, B>>, Variant, Unenforced>
                     for Bytes<Priv, $UKind, num::UInt<C, D>>
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
    impl<TKind, UKind, Variance, Transparency> BytesFromBytes<Bytes<Pub, TKind, num::UTerm>, Variance, Transparency> for Bytes<Pub, UKind, num::UTerm> {}
    impl<TKind, A, B, UKind, Variance, Transparency> BytesFromBytes<Bytes<Pub, TKind, num::UInt<A, B>>, Variance, Transparency> for Bytes<Pub, UKind, num::UTerm> {}
    impl<TKind, UKind, A, B, Variance, Transparency> BytesFromBytes<Bytes<Pub, TKind, num::UTerm>, Variance, Transparency> for Bytes<Pub, UKind, num::UInt<A, B>> {}

    /// [Bytes|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, TVis, TKind, TRest, UVis, UK, U, URest, Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Bytes<TVis, TKind, num::UTerm>, TRest>, Variance, Alignment, Transparency, Stability>
         for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        Self: FromLayout<TRest, Variance, Alignment, Transparency, Stability>,
    {}
}

mod array_to {
    use super::*;

    /// [Array|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<TVis, T, TSize, TRest, UVis, U, USize, URest, Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Array<TVis, T, TSize>, TRest>, Variance, Alignment, Transparency, Stability>
         for PCons<Array<UVis, U, USize>, URest>
    where
        PCons<Array<TVis, T, TSize>, TRest>: Flatten,
        PCons<Array<UVis, U, USize>, URest>: Flatten,

        <PCons<Array<UVis, U, USize>, URest> as Flatten>::Output:
            FromLayout<<PCons<Array<TVis, T, TSize>, TRest> as Flatten>::Output, Variance, Alignment, Transparency, Stability>,
    {}

    /// [Array|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TVis, T, TSize, TRest, UVis, UKind, USize, URest, Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Array<TVis, T, TSize>, TRest>, Variance, Alignment, Transparency, Stability>
         for PCons<Bytes<UVis, UKind, USize>, URest>
    where
        PCons<Array<TVis, T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<TVis, T, TSize>, TRest> as Flatten>::Output, Variance, Alignment, Transparency, Stability>,
    {}

    /// [Array|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, TVis, T, TSize, TRest, UK, UVis, U, URest, Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Array<TVis, T, TSize>, TRest>, Variance, Alignment, Transparency, Stability>
         for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        PCons<Array<TVis, T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<TVis, T, TSize>, TRest> as Flatten>::Output, Variance, Alignment, Transparency, Stability>,
    {}
}

mod reference_to {
    use super::*;

    /// [Reference|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<'t, TVis, T, TK, TRest, UVis, U, USize, URest, Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Variance, Alignment, Transparency, Stability>
         for PCons<Array<UVis, U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Variance, Alignment, Transparency, Stability>,
    {}

    /// [Reference|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<'t, TVis, T, TK, TRest, UVis, UKind, USize, URest, Variance, Alignment, Transparency, Stability>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Variance, Alignment, Transparency, Stability>
         for PCons<Bytes<UVis, UKind, USize>, URest>
    where
        Self: FromLayout<ReferenceBytes<TVis, TRest>, Variance, Alignment, Transparency, Stability>,
    {}

    pub trait FromMutability<T> {}
    impl FromMutability<Unique> for Unique {}
    impl FromMutability<Unique> for Shared {}
    impl FromMutability<Shared> for Shared {}

    pub trait FromAlignment<T, Stability> {}

    impl<T, U> FromAlignment<T, Stable> for U
    where
        U: Never<Increase, Alignment>,
        T: Never<Decrease, Alignment>,
    {}

    impl<T, U> FromAlignment<T, Unstable> for U {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, TVis, T, TK, TRest, UVis, U, UK, URest, Variance, Transparency, Stability>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Variance, Unchecked, Transparency, Stability>
           for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        U: Never<Increase, Alignment>,
        T: Never<Decrease, Alignment>,
        U: FromType<T, Invariant, Unchecked, Transparency, Stability>,
    {}

    /// `[Reference|_] -> [Reference|_]`
    /// ```rust
    /// use typic::private::transmute::{Stable, Variant, Static, Enforced, from_layout::FromLayout};
    /// use typic::private::bytelevel::{PCons, PNil, slot::{Pub, Priv, SharedRef}};
    /// fn can_transmute<T, U: FromLayout<T, Variant, Static, Enforced, Stable>>() {}
    ///
    /// type T = SharedRef<'static, Pub, ()>;
    /// can_transmute::<PCons<T, PNil>, PCons<T, PNil>>();
    /// ```
    #[rustfmt::skip] unsafe impl<'t, 'u, TVis, T, TK, TRest, UVis, U, UK, URest, Variance, Transparency, Stability>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Variance, Static, Transparency, Stability>
           for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        U: FromAlignment<T, Stability> + AlignedTo<T> + FromType<T, Invariant, Static, Transparency, Stability>,
    {}
}


#[cfg(test)]
mod test {
  use super::*;

  fn subsumes<T, U: FromLayout<T, Variant, Static, Enforced, Stable>>()
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
