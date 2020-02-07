use super::Layout;
use crate::private::num::*;

pub trait AlignedTo<T> {}

impl<T, U> AlignedTo<T> for U
where
    T: Layout,
    U: Layout,
    <T as Layout>::Align: PartialDiv<<U as Layout>::Align>,
{
}
