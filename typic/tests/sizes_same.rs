use static_assertions::*;
use std::mem;
use typic::{self, transmute::TransmuteFrom};

#[typic::repr(C)]
#[derive(Default)]
struct Foo(u8, u8);

#[typic::repr(C)]
#[derive(Default)]
struct Bar(u8);

const_assert_ne!(mem::size_of::<Foo>(), mem::size_of::<Bar>());

assert_not_impl_any!(Bar: TransmuteFrom<Foo>);
assert_not_impl_any!(Foo: TransmuteFrom<Bar>);
