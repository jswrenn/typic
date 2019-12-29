#![allow(dead_code)]

use crate::{padding, structure::*, transmute::Candidate, Type};
use core::mem::*;
use core::num::*;
use core::ops::*;
use frunk_core::hlist::*;
use static_assertions::*;
use typenum::operator_aliases::Sum;
use typenum::*;

pub trait Layout: Candidate {
    type Align: Unsigned;
    type Slots: SlotList;
}

impl<T> Layout for T
where
    T: Type<Padding = padding::Padded>,
    T::Representation: AlignmentOf + SlotsOf<<T::Representation as AlignmentOf>::Value, U0>,
{
    type Align = <T::Representation as AlignmentOf>::Value;
    type Slots =
        <T::Representation as SlotsOf<<T::Representation as AlignmentOf>::Value, U0>>::Slots;
}

pub trait AlignmentOf<Minimum = U1>
where
    Minimum: Unsigned,
{
    type Value: Unsigned;
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

/// An item in a `Representation`.
pub trait Slot {
    type Size: Unsigned;
}

/// A byte of initialized memory.
pub type Init = u8;

/// A slot may be byte of initialized memory.
impl Slot for Init {
    type Size = U1;
}

/// A non-zero byte of memory.
pub type NonZero = core::num::NonZeroU8;

impl Slot for NonZero {
    type Size = U1;
}

/// A byte of possibly uninitialized memory.
pub type Uninit = MaybeUninit<u8>;

/// A slot may be a byte of possibly uninitialized memory.
impl Slot for Uninit {
    type Size = U1;
}

/// A `Slot` may be a pointer.
impl<'t, T> Slot for &'t T {
    type Size = <*const T as Slot>::Size;
}

/// A `Slot` may be a pointer.
impl<'t, T> Slot for &'t mut T {
    type Size = <*const T as Slot>::Size;
}

/// A `Slot` may be a pointer.
impl<T> Slot for *const T {
    #[cfg(target_pointer_width = "64")]
    type Size = U8;
}

/// A `Slot` may be a pointer.
impl<T> Slot for *mut T {
    type Size = <*const T as Slot>::Size;
}

pub trait SlotList: HList {
    type Size: Unsigned;
}

/// ZSTs have a representation of `Empty`.
impl SlotList for HNil {
    type Size = U0;
}

impl<F, R: SlotList> SlotList for HCons<F, R>
where
    F: Slot,
    R: SlotList,

    <R as SlotList>::Size: Add<<F as Slot>::Size>,
    Sum<<R as SlotList>::Size, <F as Slot>::Size>: Unsigned,
{
    type Size = Sum<<R as SlotList>::Size, <F as Slot>::Size>;
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

      impl SlotsOf<$align, U0> for $ty {
        type Slots = <u8 as Repeat<$size>>::Output;
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

      impl SlotsOf<$align, U0> for $ty {
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

impl<'t, T> SlotsOf<U8, U0> for &'t T {
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

impl<'t, T> SlotsOf<U8, U0> for &'t mut T {
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

impl<T> SlotsOf<U8, U0> for *mut T {
    #[cfg(target_pointer_width = "64")]
    type Slots = <Init as Repeat<U8>>::Output;
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

impl<T> SlotsOf<U8, U0> for *const T {
    #[cfg(target_pointer_width = "64")]
    type Slots = <Init as Repeat<U8>>::Output;
}

impl<T> Type for *const T {
    type Padding = padding::Padded;
    type Representation = *const T;
}

/// Apply the `repr(C)` layout algorithm to find the representation of a struct.
pub trait SlotsOf<Alignment, Offset> {
    /// The representation of this struct.
    type Slots: SlotList;
}

impl<H, T, Alignment, Offset> SlotsOf<Alignment, Offset> for Fields<H, T>
where
    H: Layout,
    T: FieldList,
    (<H as Layout>::Align, Offset): Padding,

    <(<H as Layout>::Align, Offset) as Padding>::Slots:
        Add<<H as Layout>::Slots>,

    Sum<
        <(<H as Layout>::Align, Offset) as Padding>::Slots,
        <H as Layout>::Slots,
    >: SlotList,

    Offset: Add<<Sum<
              <(<H as Layout>::Align, Offset) as Padding>::Slots,
              <H as Layout>::Slots,
          > as SlotList>::Size>,

    T: SlotsOf<
        Alignment,
        Sum<
          Offset,
          <Sum<
              <(<H as Layout>::Align, Offset) as Padding>::Slots,
              <H as Layout>::Slots,
          > as SlotList>::Size>,
    >,

    Sum<<(<H as Layout>::Align, Offset) as Padding>::Slots, <H as Layout>::Slots>: Add<
        <T as SlotsOf<
            Alignment,
            Sum<
              Offset,
              <Sum<
                  <(<H as Layout>::Align, Offset) as Padding>::Slots,
                  <H as Layout>::Slots,
              > as SlotList>::Size>,
        >>::Slots,
    >,

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
              > as SlotList>::Size>,
        >>::Slots>:
    SlotList,
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
              > as SlotList>::Size>,
        >>::Slots,
    >;
}

/// After the last field, insert trailing padding.
impl<Alignment, Offset> SlotsOf<Alignment, Offset> for Empty
where
    (Alignment, Offset): Padding,
{
    type Slots = <(Alignment, Offset) as Padding>::Slots;
}

/// A padding computer for the C layout.
pub trait Padding {
    /// the number of trailing padding bytes
    type Bytes: Unsigned;

    /// the hlist representation of trailing padding bytes
    type Slots: SlotList;
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
where
    N: Unsigned,
{
    type Output: SlotList;
}

impl<T> Repeat<UTerm> for T {
    type Output = HNil;
}

impl<T, Ul: Unsigned, Bl: Bit> Repeat<UInt<Ul, Bl>> for T
where
    UInt<Ul, Bl>: Sub<B1>,
    Sub1<UInt<Ul, Bl>>: Unsigned,
    HCons<T, <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output>: SlotList,
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

assert_type_eq_all!(U8, <U5 as RoundUpTo<U8>>::Output);
assert_type_eq_all!(U0, <U0 as RoundUpTo<U8>>::Output);
assert_type_eq_all!(U8, <U8 as RoundUpTo<U8>>::Output);
assert_type_eq_all!(U16, <U9 as RoundUpTo<U8>>::Output);
