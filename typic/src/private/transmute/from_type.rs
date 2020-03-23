use crate::stability::*;
use crate::private::layout::Layout;
use crate::internal::{Public, Private};
use super::{Stable, Unstable, from_layout::FromLayout};

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait FromType<
  SourceType,
  // Can bit-validity be widened?
  Variance,
  // Is alignment checked?
  Alignment,
  // Is library safety checked?
  Transparency,
  /// Must the source and destination types have stable representations?
  Stability,
  /// Must all values of the source type be a valid instance of the destination type?
  Validity,
>{}

unsafe impl<T, U, Variance, Alignment, Transparency, Validity>
FromType<T, Variance, Alignment, Transparency, Unstable, Validity> for U
where
    T: Layout<Public>,
    U: Layout<Public>,
    <U as Layout<Public>>::ByteLevel: FromLayout<<T as Layout<Public>>::ByteLevel,
      (Variance,
      Alignment,
      Transparency,
      Unstable,
      Validity,)>
{}

unsafe impl<T, U, Variance, Alignment, Transparency, Validity>
FromType<T, Variance, Alignment, Transparency, Stable, Validity> for U
where
    T: Bound<Upper> + Layout<Public>,
    U: Bound<Lower> + Layout<Public>,

    // If stability is being enforced, then
    // the widest extent of the source type
    // must be transmutable into the narrowest
    // extent of the destination type.
    <U as Bound<Lower>>::Type:
      FromType<<T as Bound<Upper>>::Type,
        Variance,
        Alignment,
        Transparency,
        Unstable,
        Validity>,

    <U as Layout<Public>>::ByteLevel: FromLayout<<T as Layout<Public>>::ByteLevel,
      (Variance,
      Alignment,
      Transparency,
      Stable,
      Validity,)>
{}