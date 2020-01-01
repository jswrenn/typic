use static_assertions::*;
use std::mem;
use std::num::*;
use typic::{self, TransmuteFrom};

#[typic::repr(C)]
union Foo {
    a: u8,
}

#[typic::repr(C)]
union Bar {
    a: NonZeroU8,
    b: i8,
}

assert_impl_all!(Foo: TransmuteFrom<Bar>);
assert_impl_all!(Bar: TransmuteFrom<Foo>);

#[typic::repr(C)]
struct Baz(pub NonZeroU8);

assert_impl_all!(Baz: TransmuteFrom<Bar>);
assert_impl_all!(Bar: TransmuteFrom<Baz>);

assert_impl_all!(Foo: TransmuteFrom<Baz>);
assert_not_impl_any!(Baz: TransmuteFrom<Foo>);
