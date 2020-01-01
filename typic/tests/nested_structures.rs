use static_assertions::*;
use std::mem;
use typic::{self, TransmuteFrom};

#[typic::repr(C)]
#[derive(Default)]
struct Foo(u32, Bar);

#[typic::repr(C)]
#[derive(Default)]
struct Bar(u32);

#[typic::repr(C)]
#[derive(Default)]
struct Baz(u32, u32);

assert_impl_all!(Foo: TransmuteFrom<Baz>);
assert_impl_all!(Baz: TransmuteFrom<Foo>);
