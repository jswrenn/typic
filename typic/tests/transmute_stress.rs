#![recursion_limit = "512"]

use core::mem::align_of;
use static_assertions::*;
use typic::{self, TransmuteInto};

// Adapted From:
// https://rust-lang.zulipchat.com/#narrow/stream/216762-project-safe-transmute/topic/typic/near/185459723
fn stress() {
    #[typic::repr(C)]
    #[derive(Default)]
    pub struct A(
        [u64; 1],
        [u64; 2],
        [u64; 3],
        [u64; 4],
        [u64; 5],
        [u64; 6],
        [u64; 7],
        [u64; 8],
        [u64; 9],
        [u64; 10],
        [u64; 11],
        [u64; 12],
        [u64; 13],
        [u64; 14],
        [u64; 15],
        [u64; 16],
    );

    #[typic::repr(C)]
    #[derive(Default)]
    pub struct B(
        [u64; 16],
        [u64; 15],
        [u64; 14],
        [u64; 13],
        [u64; 12],
        [u64; 11],
        [u64; 10],
        [u64; 9],
        [u64; 8],
        [u64; 7],
        [u64; 6],
        [u64; 5],
        [u64; 4],
        [u64; 3],
        [u64; 2],
        [u64; 1],
    );

    let _: A = B::default().transmute_into();
}
