use crate::layout::Layout;
use typenum::*;

/// `U: AlignedTo<T>` indicates that `U`’s alignment requirement is at least as
/// strict as `T`'s, and so any memory address which satisfies the alignment of
/// `U` also satisfies the alignment of `T`.
pub trait AlignedTo<T> {}

impl<T, U> AlignedTo<T> for U
where
    T: Layout,
    U: Layout,
    <T as Layout>::Align: PartialDiv<<U as Layout>::Align>,
{
}
