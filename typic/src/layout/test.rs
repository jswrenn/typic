use super::Layout;
use crate::bytelevel::{self as blvl, slot::*};
use crate::highlevel::{self as hlvl, MaxAlign, MinAlign, Type};
use crate::num::*;
use crate::typic;
use static_assertions::*;

const _: () = {
    #[typic::repr(C)]
    struct C;

    assert_type_eq_all!(<C as Type>::ReprAlign, MinAlign);
    assert_type_eq_all!(<C as Type>::ReprPacked, MaxAlign);
    assert_type_eq_all!(<C as Type>::HighLevel, hlvl::PNil);

    assert_type_eq_all!(<C as Layout>::Align, U1);
    assert_type_eq_all!(<C as Layout>::Size, U0);
    assert_type_eq_all!(
        <C as Layout>::ByteLevel,
        blvl::PCons<PaddingSlot<U0>, blvl::PNil>
    );
};
