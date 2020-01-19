//! Type-level operations on byte-level stuff.

pub trait Add<RHS> {
    type Output;
}
pub trait Sub<RHS> {
    type Output;
}

pub type Sum<A, B> = <A as Add<B>>::Output;
pub type Diff<A, B> = <A as Sub<B>>::Output;
