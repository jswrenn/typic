use super::Layout;
use crate::num::*;

/// The amount of padding, counted in bytes, that must preceed `Self` in a
/// compound type, where `Offset` is the index of the byte following the end of
/// the preceeding field, and `Packed` is an unsigned integer reflecting the
/// minimum packing of the enclosing type.
pub trait PaddingNeededForField<Offset, Packed = <Self as Layout>::Align> {
    type Output;
}

impl<Offset, Packed, T> PaddingNeededForField<Offset, Packed> for T
where
    T: Layout,
    <T as Layout>::Align: Min<Packed>,
    Offset: PadTo<Minimum<<T as Layout>::Align, Packed>>,
{
    /// In the presence of a `repr(packed(N))` modifier, this field is packed
    /// to satisfy alignment `N` or the preferred alignment of `T`—whichever is
    /// lower.
    type Output = <Offset as PadTo<Minimum<<T as Layout>::Align, Packed>>>::Output;
}

/// The amount of padding bytes needed to align an item at offset `Self` to
/// `Align`.
pub trait PadTo<Align> {
    type Output;
}

impl<Align, Offset> PadTo<Align> for Offset
where
    Offset: RoundUpTo<Align>,
    <Offset as RoundUpTo<Align>>::Output: Sub<Offset>,
{
    type Output = Diff<<Offset as RoundUpTo<Align>>::Output, Offset>;
}
