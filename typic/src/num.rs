//! Type-level numbers.

pub use core::ops::*;
pub use typenum::{self, consts::*, operator_aliases::*, type_operators::*, uint::*, Unsigned};

pub trait SaturatingSub<RHS> {
    type Output;
}

impl<LHS, RHS> SaturatingSub<RHS> for LHS
where
    LHS: Min<RHS> + core::ops::Sub<Minimum<LHS, RHS>>,
{
    type Output = typenum::Diff<LHS, Minimum<LHS, RHS>>;
}

pub trait RoundUpTo<Multiple> {
    type Output: Unsigned;
}

impl<N, Multiple> RoundUpTo<Multiple> for N
where
    N: Add<Multiple>,
    Sum<N, Multiple>: Sub<B1>,
    Sub1<Sum<N, Multiple>>: Rem<Multiple>,
    Sub1<Sum<N, Multiple>>: Sub<Mod<Sub1<Sum<N, Multiple>>, Multiple>>,

    Diff<Sub1<Sum<N, Multiple>>, Mod<Sub1<Sum<N, Multiple>>, Multiple>>: Unsigned,
{
    type Output = Diff<Sub1<Sum<N, Multiple>>, Mod<Sub1<Sum<N, Multiple>>, Multiple>>;
}
