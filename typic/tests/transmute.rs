use core::mem::align_of;
use static_assertions::*;
use typic::{self, TransmuteInto};

#[test]
fn zst_transmute() {
    #[typic::repr(C)]
    struct T;

    #[typic::repr(C)]
    struct U;

    let _: U = T.transmute_into();
    let _: U = U.transmute_into();
    let _: T = U.transmute_into();
    let _: T = T.transmute_into();

    let _: &T = (&U).transmute_into();
    let _: &T = (&T).transmute_into();
    let _: &U = (&T).transmute_into();
    let _: &U = (&U).transmute_into();
}

#[test]
fn small_transmute() {
    #[typic::repr(C)]
    #[derive(Default)]
    struct T(u8, u8);

    #[typic::repr(C)]
    #[derive(Default)]
    struct U(u16);

    let _: U = T::default().transmute_into();
    let _: U = U::default().transmute_into();
    let _: T = U::default().transmute_into();
    let _: T = T::default().transmute_into();
}

#[test]
fn padding_transmute() {
    #[typic::repr(C)]
    #[derive(Default)]
    struct Padded(u8, u16, u8);

    #[typic::repr(C)]
    #[derive(Default)]
    struct Packed(u16, u16, u16);

    let _: Packed = Packed::default().transmute_into();
    let _: Padded = Padded::default().transmute_into();

    // Transmuting initialized bytes into padding bytes is sound.
    let _: Padded = Packed::default().transmute_into();

    // Transmuting padding bytes into initialized bytes is unsound.
    assert_not_impl_any!(Padded: TransmuteInto<Packed>);
}

#[test]
fn arrays() {
  // The inner type of the array may be mutated
  let _: [u8; 4] = [0u16; 2].transmute_into();
  let _: [u16; 2] = [0u8; 4].transmute_into();

  // Arrays may be shrunk
  let _: [u8; 4] = [0u8; 5].transmute_into();

  // Arrays may not be grown:
  assert_not_impl_any!([u8; 4]: TransmuteInto<[u8; 5]>);
  assert_not_impl_any!([u8; 4]: TransmuteInto<[u16; 4]>);
}
