use static_assertions::*;
use typic::{
    self,
    transmute::{Invariants, TransmuteFrom, Valid},
};

// This struct is not transparent, because `b` is private.
#[typic::repr(C)]
#[derive(Default)]
struct Opaque {
    a: u8,
    b: u8,
    c: u16,
}

// To safely transmute `Opaque`, the `Invariants` trait must be implemented for it manually.
unsafe impl Invariants for Opaque {
    type Error = &'static str;

    fn check(candidate: &Self::Candidate) -> Result<Valid, Self::Error>
    where
        Self: Sized,
    {
        if candidate.c % 2 == 0 {
            Ok(Valid)
        } else {
            Err("`c` must be even")
        }
    }
}

// This struct is transparent, since all fields are `pub`.
#[typic::repr(C)]
#[derive(Default)]
struct Transparent {
    pub a: u16,
    pub b: u8,
    pub c: u8,
}

// `Transparent` does not maintain any invariants on its fields.
assert_impl_all!(Transparent: Invariants);

// We may therefore freely get an instance of it from `Opaque`.
assert_impl_all!(Transparent: TransmuteFrom<Opaque>);

// The reverse is not necessarily true...
fn transparent_to_opaque() {
    let a = Transparent { a: 0, b: 0, c: 1 };
    let b = Transparent { a: 0, b: 1, c: 0 };

    #[cfg(target_endian = "little")]
    let (valid, invalid) = (a, b);

    #[cfg(target_endian = "big")]
    let (valid, invalid) = (b, a);

    assert!(Opaque::try_transmute_from(valid).is_ok());
    assert!(Opaque::try_transmute_from(invalid).is_err());
}
