use super::Layout;
use crate::private::bytelevel::{self as blvl, slot::*};
use crate::private::highlevel::{self as hlvl, MaxAlign, MinAlign, Public, Type};
use crate::private::num::*;
use crate::typic;
use static_assertions::*;

const _: () = {
    #[typic::repr(C)]
    struct C;

    assert_type_eq_all!(<C as Type>::ReprAlign, MinAlign);
    assert_type_eq_all!(<C as Type>::ReprPacked, MaxAlign);
    assert_type_eq_all!(<C as Type>::HighLevel, hlvl::PNil);

    assert_type_eq_all!(<C as Layout<Public>>::Align, U1);
    assert_type_eq_all!(<C as Layout<Public>>::Size, U0);
    assert_type_eq_all!(
        <C as Layout<Public>>::ByteLevel,
        blvl::PCons<PaddingSlot<Public, U0>, blvl::PNil>
    );
};
