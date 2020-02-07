#![feature(concat_idents)]

use core::mem::align_of;
use static_assertions::*;
use typic::{self, internal::*};

mod zst {
    use super::*;

    #[typic::repr(C)]
    struct ZST_C;

    #[typic::repr()]
    struct ZST_Rust;

    #[typic::repr(packed)]
    struct ZST_Packed;

    const_assert_eq![1, align_of::<ZST_C>()];
    const_assert_eq![1, align_of::<ZST_Rust>()];
    const_assert_eq![1, align_of::<ZST_Packed>()];

    assert_type_eq_all![
        MinAlign,
        <ZST_C as Type>::ReprAlign,
        <ZST_Rust as Type>::ReprAlign,
        <ZST_Packed as Type>::ReprAlign,
    ];

    assert_type_eq_all![
        MaxAlign,
        <ZST_C as Type>::ReprPacked,
        <ZST_Rust as Type>::ReprPacked,
        <ZST_Packed as Type>::ReprPacked,
    ];
}

mod align_1 {
    use super::*;

    #[typic::repr(align(1))]
    struct Align1;

    const_assert_eq![1, align_of::<Align1>()];

    assert_type_eq_all!(<Align1 as Type>::ReprAlign, MinAlign);
    assert_type_eq_all!(<Align1 as Type>::ReprPacked, MaxAlign);
}

mod multi_align {
    use super::*;

    #[typic::repr(align(2), align(4))]
    struct Align2_4;

    #[typic::repr(align(4), align(2))]
    struct Align4_2;

    const_assert_eq![4, align_of::<Align2_4>()];
    const_assert_eq![4, align_of::<Align4_2>()];

    assert_type_eq_all![
        U4,
        <Align2_4 as Type>::ReprAlign,
        <Align4_2 as Type>::ReprAlign,
    ];
}
