use crate::layout::Layout;
use super::from_layout::Equivalent as LayoutEquivalent;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait Equivalent<T> {}

unsafe impl<T, U> Equivalent<U> for T
where
    T: Layout,
    U: Layout,
    <U as Layout>::ByteLevel: LayoutEquivalent<<T as Layout>::ByteLevel>
{}
