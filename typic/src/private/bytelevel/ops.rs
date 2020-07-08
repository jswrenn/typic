//! Type-level operations on byte-level stuff.

use crate::private::bytelevel::slot::*;
use crate::private::bytelevel::{product::Product, PCons, PNil};
use crate::private::num;

pub trait Add<RHS> {
    type Output;
}

pub type Sum<A, B> = <A as Add<B>>::Output;

impl<Vis, K> Add<PNil> for Bytes<Vis, K, num::UTerm> {
    type Output = PNil;
}

/// `Bytes<_, N> + PNil = Bytes<_, N>`, where `N > 0`.
impl<Vis, K, A, B> Add<PNil> for Bytes<Vis, K, num::UInt<A, B>> {
    type Output = PCons<Self, PNil>;
}

impl<Vis, K, H, T> Add<PCons<H, T>> for Bytes<Vis, K, num::UTerm> {
    type Output = PCons<H, T>;
}

/// `Bytes<_, N> + PNil = Bytes<_, N>`, where `N > 0`.
impl<Vis, K, H, T, A, B> Add<PCons<H, T>> for Bytes<Vis, K, num::UInt<A, B>> {
    type Output = PCons<Self, PCons<H, T>>;
}

impl<P: Product> Add<PNil> for P {
    type Output = Self;
}

impl<RH, RT> Add<PCons<RH, RT>> for PNil {
    type Output = PCons<RH, RT>;
}

impl<RH, RT, LH, LT> Add<PCons<RH, RT>> for PCons<LH, LT>
where
    LT: Add<PCons<RH, RT>>,
{
    type Output = PCons<LH, <LT as Add<PCons<RH, RT>>>::Output>;
}
