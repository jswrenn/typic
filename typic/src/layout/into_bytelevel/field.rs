use crate::bytelevel::{slot::PaddingSlot, PCons};
use crate::layout::{Layout, PaddingNeededForField};
use crate::num;

pub trait FieldIntoByteLevel<Packed, Offset> {
    /// The padded, byte-level representation of `Self`.
    type Output;

    /// The offset immediately following `Self`.
    type Offset;

    /// The alignment of this field.
    type Align;
}

impl<Packed, Offset, F> FieldIntoByteLevel<Packed, Offset> for F
where
    F: Layout + PaddingNeededForField<Packed, Offset>,
    Offset: num::Add<<F as Layout>::Size>,
    Packed: num::Min<<F as Layout>::Align>,
{
    type Output = PCons<
        PaddingSlot<<F as PaddingNeededForField<Packed, Offset>>::Output>,
        <F as Layout>::ByteLevel,
    >;

    type Offset = num::Sum<Offset, <F as Layout>::Size>;

    type Align = num::Minimum<Packed, <F as Layout>::Align>;
}
