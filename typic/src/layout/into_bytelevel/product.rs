//! Compute the byte-level layout of a product type.
use super::field::FieldIntoByteLevel;
use crate::bytelevel;
use crate::highlevel;
use crate::layout::IntoByteLevel;
use crate::num;

impl<Align, Packed, Offset> IntoByteLevel<Align, Packed, Offset> for highlevel::PNil {
    type Output = crate::TODO;
    type Offset = Offset;
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
