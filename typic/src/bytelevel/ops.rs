//! Type-level operations on byte-level stuff.

use crate::bytelevel::slot::*;
use crate::bytelevel::{product::Product, PCons, PNil};
use crate::num;

pub trait Add<RHS> {
    type Output;
}

pub type Sum<A, B> = <A as Add<B>>::Output;

impl<K> Add<PNil> for Bytes<K, num::UTerm> {
    type Output = PNil;
}

/// `Bytes<_, N> + PNil = Bytes<_, N>`, where `N > 0`.
impl<K, A, B> Add<PNil> for Bytes<K, num::UInt<A, B>> {
    type Output = PCons<Self, PNil>;
}

impl<K, H, T> Add<PCons<H, T>> for Bytes<K, num::UTerm> {
    type Output = PCons<H, T>;
}

/// `Bytes<_, N> + PNil = Bytes<_, N>`, where `N > 0`.
impl<K, H, T, A, B> Add<PCons<H, T>> for Bytes<K, num::UInt<A, B>> {
    type Output = PCons<Self, PCons<H, T>>;
}

impl<P: Product> Add<PNil> for P {
    type Output = Self;
}

impl<RH, RT, P: Product> Add<PCons<RH, RT>> for P
where
    RT: Add<Self>,
{
    type Output = PCons<RH, <RT as Add<Self>>::Output>;
}
