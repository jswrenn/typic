//! Compute the byte-level layout of a product type.
use super::field::FieldIntoByteLevel;
use crate::bytelevel::{self, slot::PaddingSlot};
use crate::highlevel;
use crate::layout::{padding::PadTo, IntoByteLevel};
use crate::num;

#[rustfmt::skip]
impl<Align, Packed, Offset> IntoByteLevel<Align, Packed, Offset> for highlevel::PNil
where
    Offset: PadTo<Align> + num::Add<<Offset as PadTo<Align>>::Output>,
{
    type Output =
        bytelevel::PCons<
            PaddingSlot<<Offset as PadTo<Align>>::Output>,
            bytelevel::PNil
        >;

    type Offset =
        num::Sum<
            Offset,
            <Offset as PadTo<Align>>::Output
        >;

    type Align = Align;
}

#[rustfmt::skip]
impl<Align, Packed, Offset, F, R> IntoByteLevel<Align, Packed, Offset> for highlevel::PCons<F, R>
where
    F: FieldIntoByteLevel<Packed, Offset>,
    R: IntoByteLevel<Align, Packed, <F as FieldIntoByteLevel<Packed, Offset>>::Offset>,

    <F as FieldIntoByteLevel<Packed, Offset>>::Output:
        bytelevel::Add<
            <R as IntoByteLevel<
                Align,
                Packed,
                <F as FieldIntoByteLevel<Packed, Offset>>::Offset,
            >>::Output,
        >,

    <F as FieldIntoByteLevel<Packed, Offset>>::Align:
        num::Max<
            <R as IntoByteLevel<
                Align,
                Packed,
                <F as FieldIntoByteLevel<Packed, Offset>>::Offset,
            >>::Align,
        >,
{
    type Output =
        bytelevel::Sum<
            <F as FieldIntoByteLevel<Packed, Offset>>::Output,
            <R as IntoByteLevel<
                Align,
                Packed,
                <F as FieldIntoByteLevel<Packed, Offset>>::Offset,
            >>::Output,
        >;

    type Offset =
        <R as IntoByteLevel<
            Align,
            Packed,
            <F as FieldIntoByteLevel<Packed, Offset>>::Offset,
        >>::Offset;

    type Align =
        num::Maximum<
            <F as FieldIntoByteLevel<Packed, Offset>>::Align,
            <R as IntoByteLevel<
                Align,
                Packed,
                <F as FieldIntoByteLevel<Packed, Offset>>::Offset,
            >>::Align,
        >;
}
