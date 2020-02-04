use crate::num;

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