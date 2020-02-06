#![recursion_limit = "512"]

use static_assertions::*;
use typic::{self, TransmuteInto};

// Adapted From:
// https://rust-lang.zulipchat.com/#narrow/stream/216762-project-safe-transmute/topic/typic/near/185459723
fn stress() {
    #[typic::repr(C)]
    #[derive(Default)]
    pub struct A(
        pub [u64; 1],
        pub [u64; 2],
        pub [u64; 3],
        pub [u64; 4],
        pub [u64; 5],
        pub [u64; 6],
        pub [u64; 7],
        pub [u64; 8],
        pub [u64; 9],
        pub [u64; 10],
        pub [u64; 11],
        pub [u64; 12],
        pub [u64; 13],
        pub [u64; 14],
        pub [u64; 15],
        pub [u64; 16],
    );

    #[typic::repr(C)]
    #[derive(Default)]
    pub struct B(
        pub [u64; 16],
        pub [u64; 15],
        pub [u64; 14],
        pub [u64; 13],
        pub [u64; 12],
        pub [u64; 11],
        pub [u64; 10],
        pub [u64; 9],
        pub [u64; 8],
        pub [u64; 7],
        pub [u64; 6],
        pub [u64; 5],
        pub [u64; 4],
        pub [u64; 3],
        pub [u64; 2],
        pub [u64; 1],
    );

    let _: A = B::default().transmute_into();
}
