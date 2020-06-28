use crate::private::bytelevel::{slot::PaddingSlot, PCons};
use crate::private::layout::{Layout, PaddingNeededForField};
use crate::private::num::{self, Minimum, Min, Unsigned};
use crate::internal::Field;

pub trait FieldIntoByteLevel<Packed, Visibility, Offset> {
    /// The padded, byte-level representation of `Self`.
    type Output;

    /// The offset immediately following `Self`.
    type Offset: Unsigned;

    /// The alignment of this field.
    type Align: Unsigned;
}

impl<Packed, Visibility, Offset, V, F> FieldIntoByteLevel<Packed, Visibility, Offset> for Field<V, F>
where
    F: Layout<Minimum<V, Visibility>> + PaddingNeededForField<Minimum<V, Visibility>, Offset, Packed>,
    V: Min<Visibility>,
    Offset: num::Add<<F as PaddingNeededForField<Minimum<V, Visibility>, Offset, Packed>>::Output>,

    num::Sum<Offset, <F as PaddingNeededForField<Minimum<V, Visibility>, Offset, Packed>>::Output>:
      num::Add<<F as Layout<Minimum<V, Visibility>>>::Size>,

    num::Sum<
      num::Sum<Offset, <F as PaddingNeededForField<Minimum<V, Visibility>, Offset, Packed>>::Output>,
      <F as Layout<Minimum<V, Visibility>>>::Size
    >: Unsigned,

    Packed: num::Min<<F as Layout<Minimum<V, Visibility>>>::Align>,
    num::Minimum<Packed, <F as Layout<Minimum<V, Visibility>>>::Align>: Unsigned,
{
    type Output = PCons<
        PaddingSlot<Visibility, <F as PaddingNeededForField<Minimum<V, Visibility>, Offset, Packed>>::Output>,
        <F as Layout<Minimum<V, Visibility>>>::ByteLevel,
    >;

    type Offset = num::Sum<
      num::Sum<Offset, <F as PaddingNeededForField<Minimum<V, Visibility>, Offset, Packed>>::Output>,
      <F as Layout<Minimum<V, Visibility>>>::Size
    >;

    type Align = num::Minimum<Packed, <F as Layout<Minimum<V, Visibility>>>::Align>;
}
