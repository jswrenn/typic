use static_assertions::*;
use typic::{self, transmute::TransmuteFrom};

// This struct has no padding bytes.
#[typic::repr(C)]
#[derive(Default)]
struct Packed(u16, u8, u8, u32);

// This struct has two padding bytes.
#[typic::repr(C)]
#[derive(Default)]
struct Padded(u16, u32);

// `Packed` can be safely converted to `Padded`
assert_impl_all!(Padded: TransmuteFrom<Packed>);

// `Padded` cannot be safely converted to `Packed`
// Doing so would expose uninitialized memory!
assert_not_impl_any!(Packed: TransmuteFrom<Padded>);
