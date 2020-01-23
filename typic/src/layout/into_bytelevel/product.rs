//! Compute the byte-level layout of a product type.
use super::field::FieldIntoByteLevel;
use crate::bytelevel::{self, slot::PaddingSlot};
use crate::highlevel;
use crate::layout::{padding::PadTo, IntoByteLevel};
use crate::num::{self, Unsigned};

#[rustfmt::skip]
impl<ReprAlign, ReprPacked, Offset> IntoByteLevel<ReprAlign, ReprPacked, Offset> for highlevel::PNil
where
    ReprAlign: Unsigned,
    Offset: PadTo<ReprAlign> + num::Add<<Offset as PadTo<ReprAlign>>::Output>,

    num::Sum<
            Offset,
            <Offset as PadTo<ReprAlign>>::Output
        >: Unsigned,
{
    type Output =
        bytelevel::PCons<
            PaddingSlot<<Offset as PadTo<ReprAlign>>::Output>,
            bytelevel::PNil
        >;

    type Offset =
        num::Sum<
            Offset,
            <Offset as PadTo<ReprAlign>>::Output
        >;

    type Align = ReprAlign;
}

#[rustfmt::skip]
impl<ReprAlign, ReprPacked, Offset, F, R>
IntoByteLevel<ReprAlign, ReprPacked, Offset> for highlevel::PCons<F, R>
where
    F: FieldIntoByteLevel<ReprPacked, Offset>,
    R: IntoByteLevel<ReprAlign, ReprPacked, <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset>,

    <F as FieldIntoByteLevel<ReprPacked, Offset>>::Output:
        bytelevel::Add<
            <R as IntoByteLevel<
                ReprAlign,
                ReprPacked,
                <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset,
            >>::Output,
        >,

    <F as FieldIntoByteLevel<ReprPacked, Offset>>::Align:
        num::Max<
            <R as IntoByteLevel<
                ReprAlign,
                ReprPacked,
                <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset,
            >>::Align,
        >,

    num::Maximum<
        <F as FieldIntoByteLevel<ReprPacked, Offset>>::Align,
        <R as IntoByteLevel<
            ReprAlign,
            ReprPacked,
            <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset,
        >>::Align,
    >: Unsigned,

{
    type Output =
        bytelevel::Sum<
            <F as FieldIntoByteLevel<ReprPacked, Offset>>::Output,
            <R as IntoByteLevel<
                ReprAlign,
                ReprPacked,
                <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset,
            >>::Output,
        >;

    type Offset =
        <R as IntoByteLevel<
            ReprAlign,
            ReprPacked,
            <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset,
        >>::Offset;

    type Align =
        num::Maximum<
            <F as FieldIntoByteLevel<ReprPacked, Offset>>::Align,
            <R as IntoByteLevel<
                ReprAlign,
                ReprPacked,
                <F as FieldIntoByteLevel<ReprPacked, Offset>>::Offset,
            >>::Align,
        >;
}
