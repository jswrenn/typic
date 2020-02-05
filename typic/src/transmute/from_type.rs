use super::{Relax, Constrain, Safe, Sound};
use crate::layout::Layout;
use super::from_layout::FromLayout;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait FromType<T, M, S> {}

unsafe impl<T, U, M, S> FromType<T, M, S> for U
where
    T: Layout,
    U: Layout,
    <U as Layout>::ByteLevel: FromLayout<<T as Layout>::ByteLevel, M, S>
{}
