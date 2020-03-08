use crate::private::num;

use crate::private::layout::Layout;
use crate::private::bytelevel::{self as blv, slot::Array, PCons};

pub trait Flatten {
    type Output;
}

impl<Vis, T, TRest> Flatten for PCons<Array<Vis, T, num::UTerm>, TRest>
where
{
    type Output = TRest;
}

impl<Vis, T, A, B, TRest> Flatten for PCons<Array<Vis, T, num::UInt<A, B>>, TRest>
where
    T: Layout<Vis>,
    num::UInt<A, B>: num::Sub<num::B1>,

    <T as Layout<Vis>>::ByteLevel:
      blv::Add<PCons<Array<Vis, T, num::Sub1<num::UInt<A, B>>>, TRest>>,
{
    type Output =
      blv::Sum<
        <T as Layout<Vis>>::ByteLevel,
        PCons<Array<Vis, T, num::Sub1<num::UInt<A, B>>>, TRest>
      >;
}