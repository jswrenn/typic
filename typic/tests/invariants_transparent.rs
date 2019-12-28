use static_assertions::*;
use typic::{self, transmute::Invariants};

// This struct is not transparent, because `b` is private.
#[typic::repr(C)]
#[derive(Default)]
struct Opaque {
    pub a: u16,
    _b: u16,
}

// To safely transmute `Opaque`, the `Invariants` trait must be implemented for it manually.
assert_not_impl_any!(Opaque: Invariants);

// This struct is transparent, since all fields are `pub`.
#[typic::repr(C)]
#[derive(Default)]
struct Transparent {
    pub a: u16,
    pub b: u16,
}

// `Transparent` may be safely transmuted, since it does not maintain any invariants on its fields.
assert_impl_all!(Transparent: Invariants);
