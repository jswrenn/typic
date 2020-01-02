#![feature(const_int_conversion)]

use static_assertions::*;
use std::mem;
use std::num::*;
use typic::{self, TransmuteFrom};

#[typic::repr(C, i16)]
enum Foo {
  A = -1,
}

#[typic::repr(C, u16)]
enum Bar {
  A = core::u16::MAX,
}

#[typic::repr(C, u16)]
enum Baz {
  A = 0,
}

// Foo::A and Bar::A have identical bit patterns
assert_impl_all!(Foo: TransmuteFrom<Bar>);
assert_impl_all!(Bar: TransmuteFrom<Foo>);

// But nothing matches with Baz::A
assert_not_impl_any!(Foo: TransmuteFrom<Baz>);
assert_not_impl_any!(Bar: TransmuteFrom<Baz>);

#[typic::repr(C)]
enum Generic<T> {
  A(T)
}

assert_impl_all!(Generic<u8>: TransmuteFrom<Generic<u8>>);
