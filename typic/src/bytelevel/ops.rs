//! Type-level operations on byte-level stuff.

use crate::bytelevel::slot::*;
use crate::bytelevel::{PCons, PNil};
use crate::num;

pub trait Add<RHS> {
    type Output;
}

pub type Sum<A, B> = <A as Add<B>>::Output;

/// `PNil + PNil = PNil`
impl Add<PNil> for PNil {
    type Output = PNil;
}

/// `PCons + PNil = PCons`
impl<H, T> Add<PNil> for PCons<H, T> {
    type Output = Self;
}

/// `Bytes<_, U0> + PNil = PNil`
impl<K> Add<PNil> for Bytes<K, num::UTerm> {
    type Output = PNil;
}

/// `Bytes<_, N> + PNil = Bytes<_, N>`, where `N > 0`.
impl<K, A, B> Add<PNil> for Bytes<K, num::UInt<A, B>> {
    type Output = Self;
}

/// `SharedRef<'a, T> + PNil = SharedRef<'a, T>`.
impl<'a, T> Add<PNil> for SharedRef<'a, T> {
    type Output = Self;
}

/// `UniqueRef<'a, T> + PNil = UniqueRef<'a, T>`.
impl<'a, T> Add<PNil> for UniqueRef<'a, T> {
    type Output = Self;
}
