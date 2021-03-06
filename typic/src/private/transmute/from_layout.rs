use crate::private::bytelevel::{
    self as blv,
    slot::{Pub, Priv},
    slot::{bytes::kind, *},
    PCons, PNil, ReferenceBytes,
};
use crate::private::layout::{Layout, AlignedTo};
use crate::private::num::{self, UInt, UTerm};
use super::from_type::FromType;
use super::{Variant, Invariant, Static, Unchecked, Enforced, Unenforced, Stable, Unstable, AlwaysValid, MaybeInvalid};
use crate::stability::*;
mod consume;
pub use consume::Consume;

mod flatten;
pub use flatten::Flatten;

/// A marker trait implemented if the layout `T` is compatible with the layout
/// `Self`.
pub unsafe trait FromLayout<
  SourceLayout,
  Options,
> {}

/// ANYTHING -> []
unsafe impl<
  SourceLayout,
  Options
>
FromLayout<
  SourceLayout,
  Options
> for PNil {}

#[rustfmt::skip] unsafe impl<
  UVis, UKind, URest,
  Options,
>
FromLayout<PNil,
  Options
> for PCons<Bytes<UVis, UKind, UTerm>, URest>
where
    URest: FromLayout<PNil, Options>,
{}

#[rustfmt::skip] unsafe impl<
  UVis, U, URest,
  Options,
> FromLayout<PNil, Options>
         for PCons<Array<UVis, U, UTerm>, URest>
where
    URest: FromLayout<PNil, Options>,
{}


/// ```rust
/// use typic::private::transmute::{Stable, Variant, Static, Enforced, AlwaysValid, from_layout::FromLayout};
/// use typic::private::bytelevel::{PCons, PNil, slot::{InitializedSlot, Pub, Priv}};
/// use typic::private::num::U1;
/// fn can_transmute<T, U: FromLayout<T, (Variance, Static, Transparency, Stable, AlwaysValid)>, Variance, Transparency>() {}
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
        Options,
    >
    FromLayout<PCons<Bytes<TVis, TKind, TSize>, TRest>, Options>
           for PCons<Array<UVis, U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Bytes<TVis, TKind, TSize>, TRest>, Options>
    {}

    /// [Bytes|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TVis, TKind, TSize, TRest, UVis, UKind, USize, URest,
      Variance, Alignment, Transparency, Stability, Validity>
    FromLayout<PCons<Bytes<TVis, TKind, TSize>, TRest>, (Variance, Alignment, Transparency, Stability, Validity)>
           for PCons<Bytes<UVis, UKind, USize>, URest>
    where
        Bytes<UVis, UKind, USize>: BytesFromBytes<Bytes<TVis, TKind, TSize>, Variance, Transparency, Validity>,
        USize: Consume<TSize>,

        Bytes<TVis, TKind, <USize as Consume<TSize>>::TSize>: blv::Add<TRest>,
        Bytes<UVis, UKind, <USize as Consume<TSize>>::USize>: blv::Add<URest>,

        blv::Sum<Bytes<UVis, UKind, <USize as Consume<TSize>>::USize>, URest>:
          FromLayout<blv::Sum<Bytes<TVis, TKind, <USize as Consume<TSize>>::TSize>, TRest>, (Variance, Alignment, Transparency, Stability, Validity)>
    {}

    /// Implemented if a byte of `TKind` is transmutable to a byte of `Self`.
    pub trait BytesFromBytes<T, Variance, Transparency, Validity> {}

    macro_rules! constrain {
      ($($TKind: path => $UKind: path,)*) => {
        $(
          /// Regardless of variance and transparency, this `pub` to `pub` conversion is safe.
          impl<TSize, USize, Variance, Transparency, Validity>
          BytesFromBytes<Bytes<Pub,  $TKind, TSize>, Variance, Transparency, Validity>
                     for Bytes<Pub,  $UKind, USize>
          {}

          /// A `priv` to `pub` conversion is safe only if the transmutation is variant.
          impl<TSize, USize, Transparency, Validity>
          BytesFromBytes<Bytes<Priv, $TKind, TSize>, Variant, Transparency, Validity>
                     for Bytes<Pub,  $UKind, USize>
          {}

          /// A `priv`/`pub` to `priv` conversion is only safe if transparency is unchecked.
          impl<TSize, USize, TVis, Variance, Validity>
          BytesFromBytes<Bytes<TVis, $TKind, TSize>, Variance, Unenforced, Validity>
                     for Bytes<Priv, $UKind, USize>
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
          impl<TSize, USize, Transparency, Validity>
          BytesFromBytes<Bytes<Pub,  $TKind, TSize>, Variant, Transparency, Validity>
                     for Bytes<Pub,  $UKind, USize>
          {}

          /// A `priv` to `pub` conversion is safe only if the transmutation is variant.
          impl<TSize, USize, Transparency, Validity>
          BytesFromBytes<Bytes<Priv, $TKind, TSize>, Variant, Transparency, Validity>
                     for Bytes<Pub,  $UKind, USize>
          {}

          /// A `priv`/`pub` to `priv` conversion is only safe if transparency is unchecked.
          impl<TSize, USize, TVis, Validity>
          BytesFromBytes<Bytes<TVis, $TKind, TSize>, Variant, Unenforced, Validity>
                     for Bytes<Priv, $UKind, USize>
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
    // this is sketchy, but I think it's alright because of how
    // BytesFromBytes is used alongside `Consume`. Unfortunately,
    // it's been months since I last touched this code.
    // todo: refactor all of this.
    impl<A, B, Variance, Transparency, Validity>
      BytesFromBytes<Bytes<Pub, kind::Uninitialized, num::UTerm>, Variance, Transparency, Validity>
    for              Bytes<Pub, kind::Initialized, num::UInt<A, B>> {}

    // todo: wtf. why did I write this?
    // /// [Bytes|_] -> [Reference|_]
    // #[rustfmt::skip] unsafe impl<'u, TVis, TKind, TRest, UVis, UK, U, URest, Options>
    // FromLayout<PCons<Bytes<TVis, TKind, num::UTerm>, TRest>, Options>
    //      for PCons<Reference<'u, UVis, UK, U>, URest>
    // where
    //     Self: FromLayout<TRest, Options>,
    // {}
}

mod array_to {
    use super::*;

    /// [Array|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<TVis, T, TSize, TRest, UVis, U, USize, URest, Options>
    FromLayout<PCons<Array<TVis, T, TSize>, TRest>, Options>
         for PCons<Array<UVis, U, USize>, URest>
    where
        PCons<Array<TVis, T, TSize>, TRest>: Flatten,
        PCons<Array<UVis, U, USize>, URest>: Flatten,

        <PCons<Array<UVis, U, USize>, URest> as Flatten>::Output:
            FromLayout<<PCons<Array<TVis, T, TSize>, TRest> as Flatten>::Output, Options>,
    {}

    /// [Array|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<TVis, T, TSize, TRest, UVis, UKind, USize, URest, Options>
    FromLayout<PCons<Array<TVis, T, TSize>, TRest>, Options>
         for PCons<Bytes<UVis, UKind, USize>, URest>
    where
        PCons<Array<TVis, T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<TVis, T, TSize>, TRest> as Flatten>::Output, Options>,
    {}

    /// [Array|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'u, TVis, T, TSize, TRest, UK, UVis, U, URest, Options>
    FromLayout<PCons<Array<TVis, T, TSize>, TRest>, Options>
         for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        PCons<Array<TVis, T, TSize>, TRest>: Flatten,

        Self: FromLayout<<PCons<Array<TVis, T, TSize>, TRest> as Flatten>::Output, Options>,
    {}
}

mod reference_to {
    use super::*;

    /// [Reference|_] -> [Array|_]
    #[rustfmt::skip] unsafe impl<'t, TVis, T, TK, TRest, UVis, U, USize, URest, Options>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Options>
         for PCons<Array<UVis, U, USize>, URest>
    where
        Self: Flatten,
        <Self as Flatten>::Output:
          FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Options>,
    {}

    /// [Reference|_] -> [Bytes|_]
    #[rustfmt::skip] unsafe impl<'t, TVis, T, TK, TRest, UVis, UKind, USize, URest, Options>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, Options>
         for PCons<Bytes<UVis, UKind, USize>, URest>
    where
        Self: FromLayout<ReferenceBytes<TVis, TRest>, Options>,
    {}

    pub trait FromMutability<T> {}
    impl FromMutability<Unique> for Unique {}
    impl FromMutability<Unique> for Shared {}
    impl FromMutability<Shared> for Shared {}

    pub trait FromAlignment<T, Stability> {}

    impl<T, U> FromAlignment<T, Stable> for U
    where
        U: TransmutableInto,
        T: TransmutableFrom,
    {}

    impl<T, U> FromAlignment<T, Unstable> for U {}

    /// [Reference|_] -> [Reference|_]
    #[rustfmt::skip] unsafe impl<'t, 'u, TVis, T, TK, TRest, UVis, U, UK, URest, Variance, Transparency, Stability, Validity>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, (Variance, Unchecked, Transparency, Stability, Validity)>
           for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        U: FromType<T, Invariant, Unchecked, Transparency, Stability, Validity>,
    {}

    /// `[Reference|_] -> [Reference|_]`
    /// ```rust
    /// use typic::private::transmute::{Stable, Variant, Static, Enforced, AlwaysValid, from_layout::FromLayout};
    /// use typic::private::bytelevel::{PCons, PNil, slot::{Pub, Priv, SharedRef}};
    /// fn can_transmute<T, U: FromLayout<T, (Variant, Static, Enforced, Stable, AlwaysValid)>>() {}
    ///
    /// type T = SharedRef<'static, Pub, ()>;
    /// can_transmute::<PCons<T, PNil>, PCons<T, PNil>>();
    /// ```
    #[rustfmt::skip] unsafe impl<'t, 'u, TVis, T, TK, TRest, UVis, U, UK, URest, Variance, Transparency, Stability, Validity>
    FromLayout<PCons<Reference<'t, TVis, TK, T>, TRest>, (Variance, Static, Transparency, Stability, Validity)>
           for PCons<Reference<'u, UVis, UK, U>, URest>
    where
        't: 'u,
        UK: FromMutability<TK>,
        U: FromAlignment<T, Stability> + AlignedTo<T> + FromType<T, Invariant, Static, Transparency, Stability, Validity>,
    {}
}


#[cfg(test)]
mod test {
  use super::*;

  fn subsumes<T, U: FromLayout<T, (Variant, Static, Enforced, Stable, AlwaysValid)>>()
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
      P![PaddingSlot<Pub, U2>],
      P![]
    >();

    subsumes::<
      P![PaddingSlot<Pub, U2>],
      P![PaddingSlot<Pub, U1>]
    >();

    subsumes::<
      P![PaddingSlot<Pub, U1>, PaddingSlot<Pub, U1>],
      P![PaddingSlot<Pub, U2>]
    >();
  }
}
