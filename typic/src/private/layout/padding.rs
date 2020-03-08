use super::Layout;
use crate::private::num::*;

/// The amount of padding, counted in bytes, that must preceed `Self` in a
/// compound type, where `Offset` is the index of the byte following the end of
/// the preceeding field, and `Packed` is an unsigned integer reflecting the
/// minimum packing of the enclosing type.
pub trait PaddingNeededForField<Visibility, Offset, Packed> {
    type Output: Unsigned;
}

impl<Visibility, Offset, Packed, T> PaddingNeededForField<Visibility, Offset, Packed> for T
where
    T: Layout<Visibility>,
    <T as Layout<Visibility>>::Align: Min<Packed>,
    Minimum<<T as Layout<Visibility>>::Align, Packed>: Unsigned,
    Offset: PadTo<Minimum<<T as Layout<Visibility>>::Align, Packed>>,
    <Offset as PadTo<Minimum<<T as Layout<Visibility>>::Align, Packed>>>::Output: Unsigned,
{
    /// In the presence of a `repr(packed(N))` modifier, this field is packed
    /// to satisfy alignment `N` or the preferred alignment of `T`—whichever is
    /// lower.
    type Output = <Offset as PadTo<Minimum<<T as Layout<Visibility>>::Align, Packed>>>::Output;
}

/// The amount of padding bytes needed to align an item at offset `Self` to
/// `Align`.
pub trait PadTo<Align> {
    type Output: Unsigned;
}

impl<Align, Offset> PadTo<Align> for Offset
where
    Offset: RoundUpTo<Align>,
    <Offset as RoundUpTo<Align>>::Output: Sub<Offset>,
    Diff<<Offset as RoundUpTo<Align>>::Output, Offset>: Unsigned,
{
    type Output = Diff<<Offset as RoundUpTo<Align>>::Output, Offset>;
}
