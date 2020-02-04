use crate::num;
use super::from_type;

mod subsumes;
mod equivalent;

pub use subsumes::Subsumes;
pub use equivalent::Equivalent;

/// Consume `Maximum<TSize, USize>` of the leading bytes of two layouts.
pub trait Consume<TSize> {
    /// The number of bytes to append back on `TRest`.
    type TSize;

    /// The number of bytes to append back on `URest`.
    type USize;
}


#[rustfmt::skip]
impl<TSize, USize> Consume<TSize> for USize
where
    TSize: num::Min<USize>,
    TSize: num::Sub<num::Minimum<TSize, USize>>,
    USize: num::Sub<num::Minimum<TSize, USize>>,
{
    type TSize = num::Diff<TSize, num::Minimum<TSize, USize>>;
    type USize = num::Diff<USize, num::Minimum<TSize, USize>>;
}

use crate::layout::Layout;
use crate::bytelevel::{self as blv, slot::Array, PCons};

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

#[cfg(test)]
mod test {
  use super::*;

  fn subsumes<T, U: Subsumes<T>>()
  {}

  macro_rules! P {
    () => { crate::bytelevel::PNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { P![$A,] };
    ($A:ty, $($tok:tt)*) => {
        crate::bytelevel::PCons<$A, P![$($tok)*]>
    };
  }

  #[test]
  fn test() {
    use crate::typic::{self, num::*, highlevel::Type, layout::Layout};
    use crate::typic::bytelevel::slot::{bytes::kind, *};
    use static_assertions::*;
    use crate::bytelevel as blvl;

    subsumes::<
      P![PaddingSlot<U2>],
      P![]
    >();

    subsumes::<
      P![PaddingSlot<U2>],
      P![PaddingSlot<U1>]
    >();

    subsumes::<
      P![PaddingSlot<U1>, PaddingSlot<U1>],
      P![PaddingSlot<U2>]
    >();
  }
}
