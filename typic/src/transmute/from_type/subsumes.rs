use crate::layout::Layout;
use super::from_layout::Subsumes as LayoutSubsumes;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait Subsumes<T> {}

unsafe impl<T, U> Subsumes<T> for U
where
    T: Layout,
    U: Layout,
    <U as Layout>::ByteLevel: LayoutSubsumes<<T as Layout>::ByteLevel>
{}
