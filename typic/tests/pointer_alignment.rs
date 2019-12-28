use static_assertions::*;
use typic::{
    self,
    transmute::{Invariants, TransmuteFrom, Valid},
};

#[typic::repr(C)]
struct Align1(u8, u8);

#[typic::repr(C)]
struct Align2(u16);

// A pointer to a type with alignment 1 may be transmuted from a pointer to a
// type with alignment 2.
assert_impl_all!(&'static Align1: TransmuteFrom<&'static Align2>);

// The reverse is not true, since the alignment requirements of `Align2` might
// not be satisfied.
assert_not_impl_any!(&'static Align2: TransmuteFrom<&'static Align1>);
