use crate::private::num;

use crate::private::layout::Layout;
use crate::private::bytelevel::{self as blv, slot::Array, PCons};

pub trait Flatten {
    type Output;
}

impl<T, TRest> Flatten for PCons<Array<T, num::UTerm>, TRest>
where
{
    type Output = TRest;
}

impl<T, A, B, TRest> Flatten for PCons<Array<T, num::UInt<A, B>>, TRest>
where
    T: Layout,
    num::UInt<A, B>: num::Sub<num::B1>,

    <T as Layout>::ByteLevel:
      blv::Add<PCons<Array<T, num::Sub1<num::UInt<A, B>>>, TRest>>,
{
    type Output =
      blv::Sum<
        <T as Layout>::ByteLevel,
        PCons<Array<T, num::Sub1<num::UInt<A, B>>>, TRest>
      >;
}