use crate::private::bytelevel::{slot::PaddingSlot, PCons};
use crate::private::layout::{Layout, PaddingNeededForField};
use crate::private::num::{self, Unsigned};

pub trait FieldIntoByteLevel<Packed, Offset> {
    /// The padded, byte-level representation of `Self`.
    type Output;

    /// The offset immediately following `Self`.
    type Offset: Unsigned;

    /// The alignment of this field.
    type Align: Unsigned;
}

impl<Packed, Offset, F> FieldIntoByteLevel<Packed, Offset> for F
where
    F: Layout + PaddingNeededForField<Offset, Packed>,
    Offset: num::Add<<F as Layout>::Size>,
    Packed: num::Min<<F as Layout>::Align>,

    num::Sum<Offset, <F as Layout>::Size>: Unsigned,
    num::Minimum<Packed, <F as Layout>::Align>: Unsigned,
{
    type Output = PCons<
        PaddingSlot<<F as PaddingNeededForField<Offset, Packed>>::Output>,
        <F as Layout>::ByteLevel,
    >;

    type Offset = num::Sum<Offset, <F as Layout>::Size>;

    type Align = num::Minimum<Packed, <F as Layout>::Align>;
}
