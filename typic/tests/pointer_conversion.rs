use static_assertions::*;
use std::mem;
use std::num::NonZeroU64;
use typic::{self, TransmuteFrom};

// initialized bytes may be constructed from a pointer
assert_impl_all!(u64: TransmuteFrom<&'static u8>);
// the reverse is not true
assert_not_impl_any!(&'static u8: TransmuteFrom<u64>);

// smart pointers are never null, so they may be transmuted into nonzero bytes.
assert_impl_all!(NonZeroU64: TransmuteFrom<&'static u8>);

// raw pointers may be null, so they are not transmutable into nonzero bytes.
assert_not_impl_any!(NonZeroU64: TransmuteFrom<*const u8>);

// however, nonzero bytes may be transmuted into a raw pointer
assert_impl_all!(*const u8: TransmuteFrom<NonZeroU64>);
