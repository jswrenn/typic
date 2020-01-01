use crate::{hir, mir};
use core::num::*;
use core::ops::*;
use typenum::*;

pub trait Layout {
    type Align;
    type Representation: mir::Representation;
}

impl<T> Layout for T
where
    T: hir::Type,
    <T as hir::Type>::Representation:
        IntoMIR<<<T as hir::Type>::Representation as Align<U1>>::Output, U0, U0> + Align<U1>,
{
    type Align = <<T as hir::Type>::Representation as Align<U1>>::Output;
    type Representation = <<T as hir::Type>::Representation as IntoMIR<
        <<T as hir::Type>::Representation as Align<U1>>::Output,
        U0,
        U0,
    >>::Output;
}

pub trait IntoMIR<MinAlign, MinSize, Offset>
where
    MinAlign: Unsigned,
    MinSize: Unsigned,
    Offset: Unsigned,
{
    type Output: mir::Representation;
}

pub trait MinSize<MinAlign> {
    type Output: Unsigned;
}

impl<MinAlign> MinSize<MinAlign> for hir::product::Nil {
    type Output = U0;
}

impl<MinAlign, H, T> MinSize<MinAlign> for hir::product::Cons<H, T>
where
    T: hir::product::Product,
{
    type Output = U0;
}

impl<MinAlign, H> MinSize<MinAlign> for hir::coproduct::Nil<H>
where
    Self: MaxVariantSize + Align<MinAlign>,
    <Self as MaxVariantSize>::Output: RoundUpTo<<Self as Align<MinAlign>>::Output>,
{
    type Output =
        <<Self as MaxVariantSize>::Output as RoundUpTo<<Self as Align<MinAlign>>::Output>>::Output;
}

impl<MinAlign, H, T> MinSize<MinAlign> for hir::coproduct::Cons<H, T>
where
    T: hir::coproduct::Coproduct,
    Self: MaxVariantSize + Align<MinAlign>,

    <Self as MaxVariantSize>::Output: RoundUpTo<<Self as Align<MinAlign>>::Output>,
{
    type Output =
        <<Self as MaxVariantSize>::Output as RoundUpTo<<Self as Align<MinAlign>>::Output>>::Output;
}

pub trait MaxVariantSize {
    type Output: Unsigned;
}

impl<H> MaxVariantSize for hir::coproduct::Nil<H>
where
    H: Layout,
    <H as Layout>::Representation: mir::Size,
{
    type Output = <<H as Layout>::Representation as mir::Size>::Size;
}

impl<H, T> MaxVariantSize for hir::coproduct::Cons<H, T>
where
    H: Layout,
    T: hir::coproduct::Coproduct + MaxVariantSize,

    <H as Layout>::Representation: mir::Size,
    <<H as Layout>::Representation as mir::Size>::Size: Max<<T as MaxVariantSize>::Output>,

    Maximum<<<H as Layout>::Representation as mir::Size>::Size, <T as MaxVariantSize>::Output>:
        Unsigned,
{
    type Output =
        Maximum<<<H as Layout>::Representation as mir::Size>::Size, <T as MaxVariantSize>::Output>;
}

pub trait Align<MinAlign> {
    type Output: Unsigned;
}

impl<MinAlign, H> Align<MinAlign> for hir::coproduct::Nil<H>
where
    H: Layout,
    MinAlign: Unsigned,
    <H as Layout>::Align: Max<MinAlign>,
    Maximum<<H as Layout>::Align, MinAlign>: Unsigned,
{
    type Output = Maximum<<H as Layout>::Align, MinAlign>;
}

impl<MinAlign, H, T> Align<MinAlign> for hir::coproduct::Cons<H, T>
where
    MinAlign: Unsigned,
    H: Layout,
    T: hir::coproduct::Coproduct + Align<MinAlign>,
    <H as Layout>::Align: Max<<T as Align<MinAlign>>::Output>,
    Maximum<<H as Layout>::Align, <T as Align<MinAlign>>::Output>: Unsigned,
{
    type Output = Maximum<<H as Layout>::Align, <T as Align<MinAlign>>::Output>;
}

impl<MinAlign> Align<MinAlign> for hir::product::Nil
where
    MinAlign: Unsigned,
{
    type Output = MinAlign;
}

impl<MinAlign, H, T> Align<MinAlign> for hir::product::Cons<H, T>
where
    MinAlign: Unsigned,
    H: Layout,
    T: hir::product::Product + Align<MinAlign>,
    <H as Layout>::Align: Max<<T as Align<MinAlign>>::Output>,
    Maximum<<H as Layout>::Align, <T as Align<MinAlign>>::Output>: Unsigned,
{
    type Output = Maximum<<H as Layout>::Align, <T as Align<MinAlign>>::Output>;
}

// COMPOUND TYPES

impl<MinAlign, MinSize, Offset, H> IntoMIR<MinAlign, MinSize, Offset> for hir::coproduct::Nil<H>
where
    MinAlign: Unsigned,
    MinSize: Unsigned,
    Offset: Unsigned,
    H: Layout,
{
    type Output = mir::coproduct::Nil<<H as Layout>::Representation>;
}

impl<MinAlign, MinSize, Offset, H, T> IntoMIR<MinAlign, MinSize, Offset>
    for hir::coproduct::Cons<H, T>
where
    MinAlign: Unsigned,
    MinSize: Unsigned,
    Offset: Unsigned,

    H: Layout,
    T: hir::coproduct::Coproduct,
    T: IntoMIR<MinAlign, MinSize, Offset>,
    <T as IntoMIR<MinAlign, MinSize, Offset>>::Output: mir::coproduct::Coproduct,

    mir::coproduct::Cons<
        <H as Layout>::Representation,
        <T as IntoMIR<MinAlign, MinSize, Offset>>::Output,
    >: mir::Representation,
{
    type Output = mir::coproduct::Cons<
        <H as Layout>::Representation,
        <T as IntoMIR<MinAlign, MinSize, Offset>>::Output,
    >;
}

impl<MinAlign, MinSize, Offset> IntoMIR<MinAlign, MinSize, Offset> for hir::product::Nil
where
    MinAlign: Unsigned,
    MinSize: Unsigned,
    Offset: Unsigned,

    Offset: Max<MinSize>,
    Maximum<Offset, MinSize>: Sub<Offset>,
    mir::Uninit: Repeat<Diff<Maximum<Offset, MinSize>, Offset>>,

    (MinAlign, Maximum<Offset, MinSize>): Padding,
    <(MinAlign, Maximum<Offset, MinSize>) as Padding>::Slots: mir::product::Product,

    <mir::Uninit as Repeat<Diff<Maximum<Offset, MinSize>, Offset>>>::Output:
        Add<<(MinAlign, Maximum<Offset, MinSize>) as Padding>::Slots>,

    Sum<
        <mir::Uninit as Repeat<Diff<Maximum<Offset, MinSize>, Offset>>>::Output,
        <(MinAlign, Maximum<Offset, MinSize>) as Padding>::Slots,
    >: mir::product::Product,
{
    type Output = Sum<
        <mir::Uninit as Repeat<Diff<Maximum<Offset, MinSize>, Offset>>>::Output,
        <(MinAlign, Maximum<Offset, MinSize>) as Padding>::Slots,
    >;
}

impl<MinAlign, MinSize, Offset, H, T> IntoMIR<MinAlign, MinSize, Offset> for hir::product::Cons<H, T>
where
    MinAlign: Unsigned,
    MinSize: Unsigned,
    Offset: Unsigned,

    H: Layout,
    T: hir::product::Product,

    <H as Layout>::Align: Max<MinAlign>,
    Maximum<<H as Layout>::Align, MinAlign>: Unsigned,
    <H as Layout>::Representation: mir::Size,

    (<H as Layout>::Align, Offset): Padding,
    <(<H as Layout>::Align, Offset) as Padding>::Slots:
        Add<<H as Layout>::Representation>,

    Sum<
        <(<H as Layout>::Align, Offset) as Padding>::Slots,
        <H as Layout>::Representation,
    >: mir::Size,

    Offset: Add<<Sum<
              <(<H as Layout>::Align, Offset) as Padding>::Slots,
              <H as Layout>::Representation,
          > as mir::Size>::Size>,
    Sum<
      Offset,
      <Sum<
          <(<H as Layout>::Align, Offset) as Padding>::Slots,
          <H as Layout>::Representation,
      > as mir::Size>::Size>: Unsigned,

    T: IntoMIR<
        Maximum<<H as Layout>::Align, MinAlign>,
        MinSize,
        Sum<
          Offset,
          <Sum<
              <(<H as Layout>::Align, Offset) as Padding>::Slots,
              <H as Layout>::Representation,
          > as mir::Size>::Size>,
    >,

    Sum<<(<H as Layout>::Align, Offset) as Padding>::Slots, <H as Layout>::Representation>: Add<
        <T as IntoMIR<
            Maximum<<H as Layout>::Align, MinAlign>,
            MinSize,
            Sum<
              Offset,
              <Sum<
                  <(<H as Layout>::Align, Offset) as Padding>::Slots,
                  <H as Layout>::Representation,
              > as mir::Size>::Size>,
        >>::Output,
    >,

    Sum<
        Sum<<(<H as Layout>::Align, Offset) as Padding>::Slots, <H as Layout>::Representation>,
        <T as IntoMIR<
            Maximum<<H as Layout>::Align, MinAlign>,
            MinSize,
            Sum<
              Offset,
              <Sum<
                  <(<H as Layout>::Align, Offset) as Padding>::Slots,
                  <H as Layout>::Representation,
              > as mir::Size>::Size>,
        >>::Output
      >: mir::Representation,
{
    type Output =
      Sum<
        Sum<<(<H as Layout>::Align, Offset) as Padding>::Slots, <H as Layout>::Representation>,
        <T as IntoMIR<
            Maximum<<H as Layout>::Align, MinAlign>,
            MinSize,
            Sum<
              Offset,
              <Sum<
                  <(<H as Layout>::Align, Offset) as Padding>::Slots,
                  <H as Layout>::Representation,
              > as mir::Size>::Size>,
        >>::Output
      >;
}

pub trait Padding {
    /// the number of trailing padding bytes
    type Bytes: Unsigned;

    /// the hlist representation of trailing padding bytes
    type Slots: mir::product::Product;
}

impl<Alignment, Offset> Padding for (Alignment, Offset)
where
    Alignment: Unsigned,
    Offset: Unsigned,

    Offset: RoundUpTo<Alignment>,
    <Offset as RoundUpTo<Alignment>>::Output: Sub<Offset>,
    Diff<<Offset as RoundUpTo<Alignment>>::Output, Offset>: Unsigned,
    mir::Uninit: Repeat<Diff<<Offset as RoundUpTo<Alignment>>::Output, Offset>>,
{
    type Bytes = Diff<<Offset as RoundUpTo<Alignment>>::Output, Offset>;

    type Slots = <mir::Uninit as Repeat<Self::Bytes>>::Output;
}

/// Produce `N` repetitions of `Self`
pub trait Repeat<N> {
    type Output: mir::product::Product;
}

impl<T> Repeat<UTerm> for T {
    type Output = mir::product::Nil;
}

impl<T, Ul: Unsigned, Bl: Bit> Repeat<UInt<Ul, Bl>> for T
where
    T: mir::Representation,

    UInt<Ul, Bl>: Sub<B1>,
    Sub1<UInt<Ul, Bl>>: Unsigned,
    T: Repeat<<UInt<Ul, Bl> as Sub<B1>>::Output>,

    <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output: mir::product::Product,
{
    type Output = mir::product::Cons<T, <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output>;
}

pub trait RoundUpTo<Multiple>
where
    Multiple: Unsigned,
{
    type Output: Unsigned;
}

impl<N, Multiple> RoundUpTo<Multiple> for N
where
    N: Unsigned,
    Multiple: Unsigned,

    N: Add<Multiple>,
    Sum<N, Multiple>: Sub<B1>,
    Sub1<Sum<N, Multiple>>: Rem<Multiple>,
    Sub1<Sum<N, Multiple>>: Sub<Mod<Sub1<Sum<N, Multiple>>, Multiple>>,

    Diff<Sub1<Sum<N, Multiple>>, Mod<Sub1<Sum<N, Multiple>>, Multiple>>: Unsigned,
{
    type Output = Diff<Sub1<Sum<N, Multiple>>, Mod<Sub1<Sum<N, Multiple>>, Multiple>>;
}

// PRIMITIVES

macro_rules! primitive_layout {
  ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
    $(
      impl hir::Type for $ty {
        type Padding = hir::padding::Padded;
        type Representation = Self;
      }

      impl hir::Representation for $ty {}

      impl<MinAlign> Align<MinAlign> for $ty {
        type Output = $align;
      }

      impl<MinAlign> MinSize<MinAlign> for $ty
      {
        type Output = U0;
      }

      impl<MinAlign, MinSize, Offset> IntoMIR<MinAlign, MinSize, Offset> for $ty
      where
          MinAlign: Unsigned,
          MinSize: Unsigned,
          Offset: Unsigned,
      {
          type Output = <mir::Init as Repeat<$size>>::Output;
      }
    )*
  }
}

primitive_layout! {
  u8   { size: U1,   align: U1  };
  u16  { size: U2,   align: U2  };
  u32  { size: U4,   align: U4  };
  u64  { size: U8,   align: U8  };
  u128 { size: U16,  align: U16 };
  i8   { size: U1,   align: U1  };
  i16  { size: U2,   align: U2  };
  i32  { size: U4,   align: U4  };
  i64  { size: U8,   align: U8  };
  i128 { size: U16,  align: U16 };
}

#[cfg(target_pointer_width = "64")]
primitive_layout! {
  isize { size: U8, align: U8  };
  usize { size: U8, align: U8  };
}

macro_rules! nonzero_layout {
  ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
    $(
      impl hir::Type for $ty {
        type Padding = hir::padding::Padded;
        type Representation = Self;
      }

      impl hir::Representation for $ty {}

      impl<MinAlign> Align<MinAlign> for $ty {
        type Output = $align;
      }

      impl<MinAlign> MinSize<MinAlign> for $ty
      {
        type Output = U0;
      }

      impl<MinAlign, MinSize, Offset> IntoMIR<MinAlign, MinSize, Offset> for $ty
      where
          MinAlign: Unsigned,
          MinSize: Unsigned,
          Offset: Unsigned,
      {
          #[cfg(target_endian = "little")]
          type Output = mir::product::Cons<mir::NonZero, <mir::Init as Repeat<Sub1<$size>>>::Output>;
      }
    )*
  }
}

nonzero_layout! {
  NonZeroU8    { size: U1,   align: U1  };
  NonZeroU16   { size: U2,   align: U2  };
  NonZeroU32   { size: U4,   align: U4  };
  NonZeroU64   { size: U8,   align: U8  };
  NonZeroU128  { size: U16,  align: U16 };
  NonZeroI8    { size: U1,   align: U1  };
  NonZeroI16   { size: U2,   align: U2  };
  NonZeroI32   { size: U4,   align: U4  };
  NonZeroI64   { size: U8,   align: U8  };
  NonZeroI128  { size: U16,  align: U16 };
}

#[cfg(target_pointer_width = "64")]
nonzero_layout! {
  NonZeroIsize { size: U8, align: U8  };
  NonZeroUsize { size: U8, align: U8  };
}

macro_rules! group {
  ($($tokens:tt)*) => {$($tokens)*}
}

group! {
  impl<T> hir::Type for *const T {
    type Padding = hir::padding::Padded;
    type Representation = Self;
  }

  impl<T> hir::Representation for *const T {}

  impl<MinAlign, T> Align<MinAlign> for *const T {
    type Output = <usize as Align<MinAlign>>::Output;
  }

  impl<MinAlign, T> MinSize<MinAlign> for *const T
  {
    type Output = U0;
  }

  impl<MinAlign, MinSize, Offset, T> IntoMIR<MinAlign, MinSize, Offset> for *const T
  where
      MinAlign: Unsigned,
      MinSize: Unsigned,
      Offset: Unsigned,
  {
      type Output = <usize as IntoMIR<MinAlign, MinSize, Offset>>::Output;
  }
}

group! {
  impl<T> hir::Type for *mut T {
    type Padding = hir::padding::Padded;
    type Representation = Self;
  }

  impl<T> hir::Representation for *mut T {}

  impl<MinAlign, T> Align<MinAlign> for *mut T {
    type Output = <usize as Align<MinAlign>>::Output;
  }

  impl<MinAlign, T> MinSize<MinAlign> for *mut T
  {
    type Output = U0;
  }

  impl<MinAlign, MinSize, Offset, T> IntoMIR<MinAlign, MinSize, Offset> for *mut T
  where
      MinAlign: Unsigned,
      MinSize: Unsigned,
      Offset: Unsigned,
  {
      type Output = <usize as IntoMIR<MinAlign, MinSize, Offset>>::Output;
  }
}

group! {
  impl<'t, T> hir::Type for &'t T {
    type Padding = hir::padding::Padded;
    type Representation = Self;
  }

  impl<'t, T> hir::Representation for &'t T {}
  impl<'t, T> mir::Representation for &'t T {}

  impl<'t, MinAlign, T> Align<MinAlign> for &'t T {
    type Output = <usize as Align<MinAlign>>::Output;
  }

  impl<'t, MinAlign, T> MinSize<MinAlign> for &'t T
  {
    type Output = U0;
  }

  impl<'t, MinAlign, MinSize, Offset, T> IntoMIR<MinAlign, MinSize, Offset> for &'t T
  where
      MinAlign: Unsigned,
      MinSize: Unsigned,
      Offset: Unsigned,
  {
      type Output = mir::product::Cons<Self, mir::product::Nil>;
  }
}

group! {
  impl<'t, T> hir::Type for &'t mut T {
    type Padding = hir::padding::Padded;
    type Representation = Self;
  }

  impl<'t, T> hir::Representation for &'t mut T {}
  impl<'t, T> mir::Representation for &'t mut T {}

  impl<'t, MinAlign, T> Align<MinAlign> for &'t mut T {
    type Output = <usize as Align<MinAlign>>::Output;
  }

  impl<'t, MinAlign, T> MinSize<MinAlign> for &'t mut T
  {
    type Output = U0;
  }

  impl<'t, MinAlign, MinSize, Offset, T> IntoMIR<MinAlign, MinSize, Offset> for &'t mut T
  where
      MinAlign: Unsigned,
      MinSize: Unsigned,
      Offset: Unsigned,
  {
      type Output = mir::product::Cons<Self, mir::product::Nil>;
  }
}
