use crate::stability::*;
use super::IntoByteLevel;
use crate::private::bytelevel::{
    slot::{Array, InitializedSlot, SharedRef, UniqueRef},
    NonZeroSeq, PCons, PNil, ReferenceBytes,
};
use crate::private::highlevel::{MaxAlign, MinAlign};
use crate::private::highlevel::Type;
use crate::private::layout::Layout;

use crate::private::num::*;
use crate::private::target::PointerWidth;

use crate::stability::{self, TransmutableInto, TransmutableFrom};

macro_rules! primitive_layout {
    ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
        $(
            impl Type for $ty {
                #[doc(hidden)] type ReprAlign  = $align;
                #[doc(hidden)] type ReprPacked = $align;
                #[doc(hidden)] type HighLevel = Self;
            }

            unsafe impl TransmutableFrom for $ty {
                type Type = Self;
            }

            unsafe impl TransmutableInto for $ty {
                type Type = Self;
            }

            impl<ReprAlign, ReprPacked, Visibility, Offset> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for $ty
            where
                Offset: Add<$size>,
                Sum<Offset, $size>: Unsigned,
            {
                type Output = PCons<InitializedSlot<Visibility, $size>, PNil>;
                type Offset = Sum<Offset, $size>;
                type Align  = $align;
            }
        )*
    }
}

primitive_layout! {
    u8    { size: U1,           align: U1             };
    u16   { size: U2,           align: U2             };
    u32   { size: U4,           align: U4             };
    u64   { size: U8,           align: U8             };
    u128  { size: U16,          align: U16            };
    i8    { size: U1,           align: U1             };
    i16   { size: U2,           align: U2             };
    i32   { size: U4,           align: U4             };
    i64   { size: U8,           align: U8             };
    i128  { size: U16,          align: U16            };
    isize { size: PointerWidth, align: PointerWidth   };
    usize { size: PointerWidth, align: PointerWidth   };
    f32   { size: U4,           align: U4             };
    f64   { size: U8,           align: U8             };
}

use core::sync::atomic::*;

primitive_layout! {
    AtomicU8    { size: U1,           align: U1             };
    AtomicU16   { size: U2,           align: U2             };
    AtomicU32   { size: U4,           align: U4             };
    AtomicU64   { size: U8,           align: U8             };
    AtomicUsize { size: PointerWidth, align: PointerWidth   };
    AtomicI8    { size: U1,           align: U1             };
    AtomicI16   { size: U2,           align: U2             };
    AtomicI32   { size: U4,           align: U4             };
    AtomicI64   { size: U8,           align: U8             };
    AtomicIsize { size: PointerWidth, align: PointerWidth   };
}

macro_rules! nonzero_layout {
    ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
        $(
            impl Type for $ty {
                #[doc(hidden)] type ReprAlign  = $align;
                #[doc(hidden)] type ReprPacked = $align;
                #[doc(hidden)] type HighLevel = Self;
            }

            unsafe impl TransmutableFrom for $ty {
                type Type = Self;
            }

            unsafe impl TransmutableInto for $ty {
                type Type = Self;
            }

            impl<ReprAlign, ReprPacked, Visibility, Offset> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for $ty
            where
                Offset: Add<$size>,
                Sum<Offset, $size>: Unsigned,
            {
                type Output = NonZeroSeq<Visibility, $size, PNil>;
                type Offset = Sum<Offset, $size>;
                type Align  = $align;
            }
        )*
    }
}

use core::num::*;

nonzero_layout! {
    NonZeroU8    { size: U1,           align: U1             };
    NonZeroU16   { size: U2,           align: U2             };
    NonZeroU32   { size: U4,           align: U4             };
    NonZeroU64   { size: U8,           align: U8             };
    NonZeroU128  { size: U16,          align: U16            };
    NonZeroI8    { size: U1,           align: U1             };
    NonZeroI16   { size: U2,           align: U2             };
    NonZeroI32   { size: U4,           align: U4             };
    NonZeroI64   { size: U8,           align: U8             };
    NonZeroI128  { size: U16,          align: U16            };
    NonZeroIsize { size: PointerWidth, align: PointerWidth   };
    NonZeroUsize { size: PointerWidth, align: PointerWidth   };
}

#[rustfmt::skip]
impl Type for () {
    #[doc(hidden)] type ReprAlign  = PointerWidth;
    #[doc(hidden)] type ReprPacked = PointerWidth;
    #[doc(hidden)] type HighLevel = Self;
}

unsafe impl TransmutableFrom for () {
    type Type = Self;
}

unsafe impl TransmutableInto for () {
    type Type = Self;
}

impl<ReprAlign, ReprPacked, Visibility, Offset> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for ()
where
    Offset: Unsigned,
    PointerWidth: Unsigned,
{
    type Output = PNil;
    type Offset = Offset;
    type Align = PointerWidth;
}

unsafe impl<'a, T> TransmutableFrom for &'a T
{
    type Type = Self;
}

unsafe impl<'a, T> TransmutableInto for &'a T
{
    type Type = Self;
}

#[rustfmt::skip]
impl<'a, T> Type for &'a T {
    #[doc(hidden)] type ReprAlign  = PointerWidth;
    #[doc(hidden)] type ReprPacked = PointerWidth;
    #[doc(hidden)] type HighLevel = Self;
}

impl<'a, ReprAlign, ReprPacked, Visibility, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for &'a T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = PCons<SharedRef<'a, Visibility, T>, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align = PointerWidth;
}

unsafe impl<'a, T> TransmutableFrom for &'a mut T
{
    type Type = Self;
}

unsafe impl<'a, T> TransmutableInto for &'a mut T
{
    type Type = Self;
}

#[rustfmt::skip]
impl<'a, T> Type for &'a mut T {
    #[doc(hidden)] type ReprAlign  = PointerWidth;
    #[doc(hidden)] type ReprPacked = PointerWidth;
    #[doc(hidden)] type HighLevel = Self;
}

impl<'a, ReprAlign, ReprPacked, Visibility, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset>
    for &'a mut T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = PCons<UniqueRef<'a, Visibility, T>, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align = PointerWidth;
}

unsafe impl<T> TransmutableFrom for *const T {
    type Type = Self;
}

unsafe impl<T> TransmutableInto for *const T {
    type Type = Self;
}

#[rustfmt::skip]
impl<T> Type for *const T {
    #[doc(hidden)] type ReprAlign  = PointerWidth;
    #[doc(hidden)] type ReprPacked = PointerWidth;
    #[doc(hidden)] type HighLevel = Self;
}

impl<ReprAlign, ReprPacked, Visibility, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for *const T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = ReferenceBytes<Visibility, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align = PointerWidth;
}

unsafe impl<T> TransmutableFrom for *mut T {
    type Type = Self;
}

unsafe impl<T> TransmutableInto for *mut T {
    type Type = Self;
}

#[rustfmt::skip]
impl<T> Type for *mut T {
    #[doc(hidden)] type ReprAlign  = PointerWidth;
    #[doc(hidden)] type ReprPacked = PointerWidth;
    #[doc(hidden)] type HighLevel = Self;
}

impl<ReprAlign, ReprPacked, Visibility, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for *mut T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = ReferenceBytes<Visibility, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align = PointerWidth;
}

#[rustfmt::skip]
impl<T> Type for AtomicPtr<T> {
    #[doc(hidden)] type ReprAlign  = PointerWidth;
    #[doc(hidden)] type ReprPacked = PointerWidth;
    #[doc(hidden)] type HighLevel = Self;
}

unsafe impl<T> TransmutableFrom for AtomicPtr<T> {
    type Type = Self;
}

unsafe impl<T> TransmutableInto for AtomicPtr<T> {
    type Type = Self;
}

impl<ReprAlign, ReprPacked, Visibility, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for AtomicPtr<T>
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = ReferenceBytes<Visibility, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align = PointerWidth;
}

use core::cell::{Cell, UnsafeCell};

#[rustfmt::skip]
impl<T> Type for Cell<T>
where
    T: Type,
{
    #[doc(hidden)] type ReprAlign  = <T as Type>::ReprAlign;
    #[doc(hidden)] type ReprPacked = <T as Type>::ReprPacked;
    #[doc(hidden)] type HighLevel =  <T as Type>::HighLevel;
}

unsafe impl<T: TransmutableFrom> TransmutableFrom for Cell<T>
{
    type Type = <T as TransmutableFrom>::Type;
}

unsafe impl<T: TransmutableInto> TransmutableInto for Cell<T>
{
    type Type = <T as TransmutableInto>::Type;
}

#[rustfmt::skip]
impl<T> Type for UnsafeCell<T>
where
    T: Type,
{
    #[doc(hidden)] type ReprAlign  = <T as Type>::ReprAlign;
    #[doc(hidden)] type ReprPacked = <T as Type>::ReprPacked;
    #[doc(hidden)] type HighLevel =  <T as Type>::HighLevel;
}

unsafe impl<T: TransmutableInto> TransmutableInto for UnsafeCell<T>
{
    type Type = <T as TransmutableInto>::Type;
}

unsafe impl<T: TransmutableFrom> TransmutableFrom for UnsafeCell<T>
{
    type Type = <T as TransmutableFrom>::Type;
}

macro_rules! array_layout {
  ($($n: expr, $t: ty);*) => {
    $(
        impl<T> Type for [T; $n] {
            #[doc(hidden)] type ReprAlign  = MinAlign;
            #[doc(hidden)] type ReprPacked = MaxAlign;
            #[doc(hidden)] type HighLevel = Self;
        }

        unsafe impl<T: TransmutableFrom> TransmutableFrom for [T; $n]
        where
            [<T as TransmutableFrom>::Type; $n]: Layout
        {
            type Type = [<T as TransmutableFrom>::Type; $n];
        }

        unsafe impl<T: TransmutableInto> TransmutableInto for [T; $n]
        where
            [<T as TransmutableInto>::Type; $n]: Layout
        {
            type Type = [<T as TransmutableInto>::Type; $n];
        }

        impl<ReprAlign, ReprPacked, Visibility, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset>
            for [T; $n]
        where
            T: Layout<Visibility>,
            $t: Mul<<T as Layout<Visibility>>::Size>,

            Offset: Add<Prod<$t, <T as Layout<Visibility>>::Size>>,
            Sum<Offset, Prod<$t, <T as Layout<Visibility>>::Size>>: Unsigned,
        {
            type Output = PCons<Array<Visibility, T, $t>, PNil>;
            type Offset = Sum<Offset, Prod<$t, <T as Layout<Visibility>>::Size>>;
            type Align = <T as Layout<Visibility>>::Align;
        }
    )*
  };
}

array_layout![
   0,  U0;
   1,  U1;
   2,  U2;
   3,  U3;
   4,  U4;
   5,  U5;
   6,  U6;
   7,  U7;
   8,  U8;
   9,  U9;
  10, U10;
  11, U11;
  12, U12;
  13, U13;
  14, U14;
  15, U15;
  16, U16;
  17, U17;
  18, U18;
  19, U19;
  20, U20;
  21, U21;
  22, U22;
  23, U23;
  24, U24;
  25, U25;
  26, U26;
  27, U27;
  28, U28;
  29, U29;
  30, U30;
  31, U31;
  32, U32
];

use generic_array::{GenericArray, ArrayLength};

impl<T, N> Type for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[doc(hidden)] type ReprAlign  = MinAlign;
    #[doc(hidden)] type ReprPacked = MaxAlign;
    #[doc(hidden)] type HighLevel = Self;
}

unsafe impl<T, N> TransmutableFrom for GenericArray<T, N>
where
    N: ArrayLength<T> + ArrayLength<<T as TransmutableFrom>::Type>,
    T: TransmutableFrom,
    GenericArray<<T as TransmutableFrom>::Type, N>: Layout,
{
    type Type = GenericArray<<T as TransmutableFrom>::Type, N>;
}

unsafe impl<T, N> TransmutableInto for GenericArray<T, N>
where
    N: ArrayLength<T> + ArrayLength<<T as TransmutableInto>::Type>,
    T: TransmutableInto,
    GenericArray<<T as TransmutableInto>::Type, N>: Layout,
{
    type Type = GenericArray<<T as TransmutableInto>::Type, N>;
}

impl<ReprAlign, ReprPacked, Visibility, Offset, T, N> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset>
    for GenericArray<T, N>
where
    T: Layout<Visibility>,
    N: ArrayLength<T>,
    N: Mul<<T as Layout<Visibility>>::Size>,

    Offset: Add<Prod<N, <T as Layout<Visibility>>::Size>>,
    Sum<Offset, Prod<N, <T as Layout<Visibility>>::Size>>: Unsigned,
{
    type Output = PCons<Array<Visibility, T, N>, PNil>;
    type Offset = Sum<Offset, Prod<N, <T as Layout<Visibility>>::Size>>;
    type Align = <T as Layout<Visibility>>::Align;
}
