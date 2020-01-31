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
    TSize: num::Max<USize>,
    TSize: num::Sub<num::Maximum<TSize, USize>>,
    USize: num::Sub<num::Maximum<TSize, USize>>,
{
    type TSize = num::Diff<TSize, num::Maximum<TSize, USize>>;
    type USize = num::Diff<USize, num::Maximum<TSize, USize>>;
}

