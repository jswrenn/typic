use super::Layout;
use crate::internal::Public;
use crate::private::num::*;

pub trait AlignedTo<T> {}

impl<T, U> AlignedTo<T> for U
where
    T: Layout<Public>,
    U: Layout<Public>,
    <T as Layout<Public>>::Align: PartialDiv<<U as Layout<Public>>::Align>,
{
}
