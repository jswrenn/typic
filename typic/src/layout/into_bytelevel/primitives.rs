use super::IntoByteLevel;
use crate::bytelevel::{slot::InitializedSlot, PCons, PNil};
use crate::highlevel::Type;

use crate::num::*;
use crate::target::PointerWidth;

macro_rules! primitive_layout {
    ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
        $(
            impl Type for $ty {
                type Align  = $align;
                type Packed = $align;
                type HighLevel = Self;
            }

            impl<Align, Packed, Offset> IntoByteLevel<Align, Packed, Offset> for $ty
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
