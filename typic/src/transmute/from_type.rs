use super::{Relax, Constrain};
use crate::layout::Layout;
use super::from_layout::FromLayout;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait FromType<T, M> {}

unsafe impl<T, U, M> FromType<T, M> for U
where
    T: Layout,
    U: Layout,
    <U as Layout>::ByteLevel: FromLayout<<T as Layout>::ByteLevel, M>
{}
