#![allow(dead_code)]

use crate::{padding, structure::*, transmute::Candidate, Type};
use core::mem::*;
use core::num::*;
use core::ops::*;
use frunk_core::hlist::*;
use frunk_core::coproduct::*;
use static_assertions::*;
use typenum::operator_aliases::Sum;
use typenum::*;

pub trait Layout {
    type Align;
    type Slots;
}

impl<T> Layout for T
where
    T: Type<Padding = padding::Padded>,
    T::Representation
      : AlignmentOf
      + MinimumSize
      + SlotsOf<
          <T::Representation as AlignmentOf>::Value,
          U0,
          <T::Representation as MinimumSize>::Size,
        >,
{
    type Align = <T::Representation as AlignmentOf>::Value;
    type Slots =
        <T::Representation as SlotsOf<
          <T::Representation as AlignmentOf>::Value,
          U0,
          <T::Representation as MinimumSize>::Size,
        >>::Slots;
}

pub trait AlignmentOf<Minimum = U1>
{
    type Value;
}

impl<Minimum> AlignmentOf<Minimum> for Empty
where
    Minimum: Unsigned,
{
    type Value = Minimum;
}

impl<F, R, Minimum> AlignmentOf<Minimum> for Fields<F, R>
where
    F: Layout,
    R: FieldList + AlignmentOf<Minimum>,
    Minimum: Unsigned,
    <F as Layout>::Align: Max<<R as AlignmentOf<Minimum>>::Value>,
    Maximum<<F as Layout>::Align, <R as AlignmentOf<Minimum>>::Value>: Unsigned,
{
    type Value = Maximum<<F as Layout>::Align, <R as AlignmentOf<Minimum>>::Value>;
}

/// A byte of initialized memory.
pub type Init = u8;

/// A slot may be byte of initialized memory.
impl Size for Init {
    type Size = U1;
}

/// A non-zero byte of memory.
pub type NonZero = core::num::NonZeroU8;

impl Size for NonZero {
    type Size = U1;
}

/// A byte of possibly uninitialized memory.
pub type Uninit = MaybeUninit<u8>;

/// A slot may be a byte of possibly uninitialized memory.
impl Size for Uninit {
    type Size = U1;
}

/// A `Slot` may be a pointer.
impl<'t, T> Size for &'t T {
    type Size = <*const T as Size>::Size;
}

/// A `Slot` may be a pointer.
impl<'t, T> Size for &'t mut T {
    type Size = <*const T as Size>::Size;
}

/// A `Slot` may be a pointer.
impl<T> Size for *const T {
    #[cfg(target_pointer_width = "64")]
    type Size = U8;
}

/// A `Slot` may be a pointer.
impl<T> Size for *mut T {
    type Size = <*const T as Size>::Size;
}

pub trait Size {
    type Size;
}

impl Size for HNil {
    type Size = U0;
}

impl<H, T> Size for HCons<H, T>
where
  H: Size,
  T: Size,
  <T as Size>::Size: Add<<H as Size>::Size>,
{
    type Size = Sum<<T as Size>::Size, <H as Size>::Size>;
}

impl Size for CNil {
    type Size = U0;
}

impl<L, R> Size for Coproduct<L, R>
where
  L: Size,
  R: Size,
  <R as Size>::Size: Max<<L as Size>::Size>,
{
    type Size = Maximum<<R as Size>::Size, <L as Size>::Size>;
}

macro_rules! primitive_layout {
  ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
    $(
      impl Candidate for $ty {
        type Candidate = Self;
      }

      impl AlignmentOf<U1> for $ty {
        type Value = $align;
      }

      impl SlotsOf<$align, U0, U0> for $ty {
        type Slots = <u8 as Repeat<$size>>::Output;
      }

      impl MinimumSize for $ty {
        type Size = U0;
      }

      impl Type for $ty {
        type Padding = padding::Padded;
        type Representation = $ty;
      }
    )*
  };
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

macro_rules! nonzero_layout {
  ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
    $(
      impl Candidate for $ty {
        type Candidate = Self;
      }

      impl AlignmentOf<U1> for $ty {
        type Value = $align;
      }

      impl MinimumSize for $ty {
        type Size = U0;
      }

      impl SlotsOf<$align, U0, U0> for $ty {
        #[cfg(target_endian = "little")]
        type Slots = HCons<NonZero, <Init as Repeat<Sub1<$size>>>::Output>;
      }

      impl Type for $ty {
        type Padding = padding::Padded;
        type Representation = $ty;
      }
    )*
  };
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

impl<'t, T> Candidate for &'t T {
    type Candidate = Self;
}

impl<'t, T, N: Unsigned> AlignmentOf<N> for &'t T {
    type Value = U8;
}

impl<'t, T> MinimumSize for &'t T {
    type Size = U0;
}

impl<'t, T> SlotsOf<U8, U0, U0> for &'t T {
    type Slots = HCons<Self, HNil>;
}

impl<'t, T> Type for &'t T {
    type Padding = padding::Padded;
    type Representation = &'t T;
}

impl<'t, T> Candidate for &'t mut T {
    type Candidate = Self;
}

impl<'t, T, N: Unsigned> AlignmentOf<N> for &'t mut T {
    type Value = U8;
}

impl<'t, T> MinimumSize for &'t mut T {
    type Size = U0;
}

impl<'t, T> SlotsOf<U8, U0, U0> for &'t mut T {
    type Slots = HCons<Self, HNil>;
}

impl<'t, T> Type for &'t mut T {
    type Padding = padding::Padded;
    type Representation = &'t mut T;
}

impl<T> Candidate for *mut T {
    type Candidate = Self;
}

impl<T, N: Unsigned> AlignmentOf<N> for *mut T {
    type Value = U8;
}

impl<T> SlotsOf<U8, U0, U0> for *mut T {
    #[cfg(target_pointer_width = "64")]
    type Slots = <Init as Repeat<U8>>::Output;
}

impl<T> MinimumSize for *mut T {
    type Size = U0;
}

impl<T> Type for *mut T {
    type Padding = padding::Padded;
    type Representation = *mut T;
}

impl<T> Candidate for *const T {
    type Candidate = Self;
}

impl<T, N: Unsigned> AlignmentOf<N> for *const T {
    type Value = U8;
}

impl<T> MinimumSize for *const T {
    type Size = U0;
}

impl<T> SlotsOf<U8, U0, U0> for *const T {
    #[cfg(target_pointer_width = "64")]
    type Slots = <Init as Repeat<U8>>::Output;
}

impl<T> Type for *const T {
    type Padding = padding::Padded;
    type Representation = *const T;
}

/// Apply the `repr(C)` layout algorithm to find the representation of a struct.
pub trait SlotsOf<Alignment, Offset, MinimumSize> {
    /// The representation of this struct.
    type Slots;
}

pub trait PadVariant<Alignment, MinimumSize> {
  type Slots;
}


impl<L, R, Alignment, Offset, MinimumSize> SlotsOf<Alignment, Offset, MinimumSize> for Variants<L, R>
where
  L: SlotsOf<
    Alignment,
    U0,
    U0,
  >,
  R: SlotsOf<
    Alignment,
    U0,
    MinimumSize,
  >,
{
    type Slots =
      Coproduct<
        <L as SlotsOf<
          Alignment,
          U0,
          U0,
        >>::Slots,
        <R as SlotsOf<
          Alignment,
          U0,
          MinimumSize,
        >>::Slots,
      >;
}

impl<Alignment, Offset, MinimumSize> SlotsOf<Alignment, Offset, MinimumSize> for None
where
{
    type Slots = None;
}


impl<H, T, Alignment, Offset, MinimumSize> SlotsOf<Alignment, Offset, MinimumSize> for Fields<H, T>
where
    H: Layout,
    T: FieldList,
    (<H as Layout>::Align, Offset): Padding,

    <(<H as Layout>::Align, Offset) as Padding>::Slots:
        Add<<H as Layout>::Slots>,

    Sum<
        <(<H as Layout>::Align, Offset) as Padding>::Slots,
        <H as Layout>::Slots,
    >: Size,

    Offset: Add<<Sum<
              <(<H as Layout>::Align, Offset) as Padding>::Slots,
              <H as Layout>::Slots,
          > as Size>::Size>,

    T: SlotsOf<
        Alignment,
        Sum<
          Offset,
          <Sum<
              <(<H as Layout>::Align, Offset) as Padding>::Slots,
              <H as Layout>::Slots,
          > as Size>::Size>,
        MinimumSize
    >,

    Sum<<(<H as Layout>::Align, Offset) as Padding>::Slots, <H as Layout>::Slots>: Add<
        <T as SlotsOf<
            Alignment,
            Sum<
              Offset,
              <Sum<
                  <(<H as Layout>::Align, Offset) as Padding>::Slots,
                  <H as Layout>::Slots,
              > as Size>::Size>,
            MinimumSize
        >>::Slots,
    >,
{
    /// `[Uninit; offset % field.alignment] + [H repr] + [T repr]`
    type Slots =
      // repr of padded `H` + repr of the rest of this structure
      Sum<
        // padding + `H` field repr
        Sum<
          // padding bytes
          <(<H as Layout>::Align, Offset) as Padding>::Slots,
          // `H` repr bytes
          <H as Layout>::Slots,
        >,
        // repr bytes of the rest of this structure
        <T as SlotsOf<
            Alignment,
            // the offset increases by (padding + `H` field repr) bytes.
            Sum<
              Offset,
              <Sum<
                  <(<H as Layout>::Align, Offset) as Padding>::Slots,
                  <H as Layout>::Slots,
              > as Size>::Size>,
            MinimumSize
        >>::Slots,
    >;
}

/// After the last field, insert trailing padding.
impl<Alignment, Offset, MinimumSize> SlotsOf<Alignment, Offset, MinimumSize> for Empty
where
    Offset: Max<MinimumSize>,
    Maximum<Offset, MinimumSize>: Sub<Offset>,
    Uninit: Repeat<Diff<Maximum<Offset, MinimumSize>, Offset>>,
    <Uninit as Repeat<Diff<Maximum<Offset, MinimumSize>, Offset>>>::Output:
      Add<<(Alignment, Maximum<Offset, MinimumSize>) as Padding>::Slots>,
    (Alignment, Maximum<Offset, MinimumSize>): Padding,
{
    type Slots =
      Sum<
        <Uninit as Repeat<Diff<Maximum<Offset, MinimumSize>, Offset>>>::Output,
        <(Alignment, Maximum<Offset, MinimumSize>) as Padding>::Slots>;
}

/// A padding computer for the C layout.
pub trait Padding {
    /// the number of trailing padding bytes
    type Bytes: Unsigned;

    /// the hlist representation of trailing padding bytes
    type Slots;
}

impl<Alignment, Offset> Padding for (Alignment, Offset)
where
    Offset: RoundUpTo<Alignment>,
    <Offset as RoundUpTo<Alignment>>::Output: Sub<Offset>,
    Diff<<Offset as RoundUpTo<Alignment>>::Output, Offset>: Unsigned,
    Uninit: Repeat<Diff<<Offset as RoundUpTo<Alignment>>::Output, Offset>>,
{
    type Bytes = Diff<<Offset as RoundUpTo<Alignment>>::Output, Offset>;

    type Slots = <Uninit as Repeat<Self::Bytes>>::Output;
}

/// Produce `N` repetitions of `Self`
pub trait Repeat<N>
{
    type Output: Size;
}

impl<T> Repeat<UTerm> for T {
    type Output = HNil;
}

impl<T, Ul: Unsigned, Bl: Bit> Repeat<UInt<Ul, Bl>> for T
where
    UInt<Ul, Bl>: Sub<B1>,
    Sub1<UInt<Ul, Bl>>: Unsigned,
    HCons<T, <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output>: Size,
    T: Repeat<<UInt<Ul, Bl> as Sub<B1>>::Output>,
{
    type Output = HCons<T, <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output>;
}

/// Round `Self` to the nearest multiple of `N`.
pub trait RoundUpTo<Multiple> {
    type Output;
}

impl<N, Multiple> RoundUpTo<Multiple> for N
where
    N: Add<Multiple>,
    Sum<N, Multiple>: Sub<B1>,
    Sub1<Sum<N, Multiple>>: Rem<Multiple>,
    Sub1<Sum<N, Multiple>>: Sub<Mod<Sub1<Sum<N, Multiple>>, Multiple>>,
{
    type Output = Diff<Sub1<Sum<N, Multiple>>, Mod<Sub1<Sum<N, Multiple>>, Multiple>>;
}

use frunk_core::Coprod;

type Union = Coprod![u32, u32];

//assert_type_eq_all!(U4, <Union as UnionSize>::Size);

assert_type_eq_all!(U8, <U5 as RoundUpTo<U8>>::Output);
assert_type_eq_all!(U0, <U0 as RoundUpTo<U8>>::Output);
assert_type_eq_all!(U8, <U8 as RoundUpTo<U8>>::Output);
assert_type_eq_all!(U16, <U9 as RoundUpTo<U8>>::Output);


pub trait MaxVariantSize {
  type Size;
}

impl MaxVariantSize for None {
  type Size = U0;
}

impl<L, R> MaxVariantSize for Variants<L, R>
where
  Self: AlignmentOf,
  L: SlotsOf<
    <Self as AlignmentOf>::Value,
    U0,
    U0,
  >,
  <L as SlotsOf<
    <Self as AlignmentOf>::Value,
    U0,
    U0,
  >>::Slots: Size,
  R: MaxVariantSize,
  
  <R as MaxVariantSize>::Size: Max<<<L as SlotsOf<
    <Self as AlignmentOf>::Value,
    U0,
    U0,
  >>::Slots as Size>::Size>

{
    type Size =
      Maximum<
        <R as MaxVariantSize>::Size,
        <<L as SlotsOf<
          <Self as AlignmentOf>::Value,
          U0,
          U0,
        >>::Slots as Size>::Size
      >;
}

use frunk_core::coproduct::*;

impl<N> AlignmentOf<N> for CNil {
  type Value = U1;
}

impl<L, R, N> AlignmentOf<N> for Coproduct<L, R>
where
  //R: AlignmentOf<N>,
  //<R as AlignmentOf<N>>::Value: Max<<L as Layout>::Align>,
{
    type Value = U1;//Maximum<<R as AlignmentOf<N>>::Value, <L as Layout>::Align>;
}

pub trait MinimumSize
{
  type Size;
}


impl MinimumSize for Empty {
  type Size = U0;
}

impl<H, T> MinimumSize for Fields<H, T> {
  type Size = U0;
}

impl MinimumSize for None {
  type Size = U0;
}

impl<L, R> MinimumSize for Variants<L, R>
where
  Self: MaxVariantSize + AlignmentOf,
  <Self as MaxVariantSize>::Size: RoundUpTo<<Self as AlignmentOf>::Value>
{
  type Size = <<Self as MaxVariantSize>::Size as RoundUpTo<<Self as AlignmentOf>::Value>>::Output;
}
