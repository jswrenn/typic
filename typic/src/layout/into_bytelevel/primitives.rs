use super::IntoByteLevel;
use crate::bytelevel::{slot::{InitializedSlot, SharedRef, UniqueRef}, NonZeroSeq, PCons, PNil, ReferenceBytes};
use crate::highlevel::Type;

use crate::num::*;
use crate::target::PointerWidth;

macro_rules! primitive_layout {
    ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
        $(
            impl Type for $ty {
                type ReprAlign  = $align;
                type ReprPacked = $align;
                type HighLevel = Self;
            }

            impl<ReprAlign, ReprPacked, Offset> IntoByteLevel<ReprAlign, ReprPacked, Offset> for $ty
            where
                Offset: Add<$size>,
                Sum<Offset, $size>: Unsigned,
            {
                type Output = PCons<InitializedSlot<$size>, PNil>;
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
                type ReprAlign  = $align;
                type ReprPacked = $align;
                type HighLevel = Self;
            }

            impl<ReprAlign, ReprPacked, Offset> IntoByteLevel<ReprAlign, ReprPacked, Offset> for $ty
            where
                Offset: Add<$size>,
                Sum<Offset, $size>: Unsigned,
            {
                type Output = NonZeroSeq<$size, PNil>;
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

impl<'a, T> Type for &'a T {
    type ReprAlign  = PointerWidth;
    type ReprPacked = PointerWidth;
    type HighLevel = Self;
}

impl<'a, ReprAlign, ReprPacked, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Offset> for &'a T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = PCons<SharedRef<'a, T>, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align  = PointerWidth;
}

impl<'a, T> Type for &'a mut T {
    type ReprAlign  = PointerWidth;
    type ReprPacked = PointerWidth;
    type HighLevel = Self;
}

impl<'a, ReprAlign, ReprPacked, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Offset> for &'a mut T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = PCons<UniqueRef<'a, T>, PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align  = PointerWidth;
}

impl<T> Type for *const T {
    type ReprAlign  = PointerWidth;
    type ReprPacked = PointerWidth;
    type HighLevel = Self;
}

impl<ReprAlign, ReprPacked, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Offset> for *const T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = ReferenceBytes<PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align  = PointerWidth;
}

impl<T> Type for *mut T {
    type ReprAlign  = PointerWidth;
    type ReprPacked = PointerWidth;
    type HighLevel = Self;
}

impl<ReprAlign, ReprPacked, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Offset> for *mut T
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = ReferenceBytes<PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align  = PointerWidth;
}

impl<T> Type for AtomicPtr<T> {
    type ReprAlign  = PointerWidth;
    type ReprPacked = PointerWidth;
    type HighLevel = Self;
}

impl<ReprAlign, ReprPacked, Offset, T> IntoByteLevel<ReprAlign, ReprPacked, Offset> for AtomicPtr<T>
where
    Offset: Add<PointerWidth>,
    Sum<Offset, PointerWidth>: Unsigned,
{
    type Output = ReferenceBytes<PNil>;
    type Offset = Sum<Offset, PointerWidth>;
    type Align  = PointerWidth;
}
