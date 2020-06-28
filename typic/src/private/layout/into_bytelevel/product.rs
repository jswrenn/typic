//! Compute the byte-level layout of a product type.
use super::field::FieldIntoByteLevel;
use crate::private::bytelevel::{self, slot::PaddingSlot};
use crate::private::highlevel;
use crate::private::layout::{padding::PadTo, IntoByteLevel};
use crate::private::num::{self, Unsigned};

#[rustfmt::skip]
impl<ReprAlign, ReprPacked, Visibility, Offset> IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for highlevel::PNil
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
impl<ReprAlign, ReprPacked, Visibility, Offset, F, R>
IntoByteLevel<ReprAlign, ReprPacked, Visibility, Offset> for highlevel::PCons<F, R>
where
    F: FieldIntoByteLevel<ReprPacked, Visibility, Offset>,
    R: IntoByteLevel<num::Maximum<
        <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align,
        ReprAlign,
    >, ReprPacked, Visibility, <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Offset>,

    <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Output:
        bytelevel::Add<
            <R as IntoByteLevel<
                num::Maximum<
                    <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align,
                    ReprAlign,
                >,
                ReprPacked,
                Visibility,
                <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Offset,
            >>::Output,
        >,

    <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align:
        num::Max<ReprAlign>,

    num::Maximum<
        <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align,
        ReprAlign,
    >: Unsigned,
{
    type Output =
        bytelevel::Sum<
            <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Output,
            <R as IntoByteLevel<
                num::Maximum<
                    <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align,
                    ReprAlign,
                >,
                ReprPacked,
                Visibility,
                <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Offset,
            >>::Output,
        >;

    type Offset =
        <R as IntoByteLevel<
            num::Maximum<
                <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align,
                ReprAlign,
            >,
            ReprPacked,
            Visibility,
            <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Offset,
        >>::Offset;

    type Align =
        num::Maximum<
            <F as FieldIntoByteLevel<ReprPacked, Visibility, Offset>>::Align,
            ReprAlign,
        >;
}
