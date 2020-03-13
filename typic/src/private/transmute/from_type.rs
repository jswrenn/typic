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
>{}

unsafe impl<T, U, Variance, Alignment, Transparency>
FromType<T, Variance, Alignment, Transparency, Unstable> for U
where
    T: Layout<Public>,
    U: Layout<Public>,
    <U as Layout<Public>>::ByteLevel: FromLayout<<T as Layout<Public>>::ByteLevel,
      Variance,
      Alignment,
      Transparency,
      Unstable>
{}

unsafe impl<T, U, Variance, Alignment, Transparency>
FromType<T, Variance, Alignment, Transparency, Stable> for U
where
    T: Never<Increase, Size> + Layout<Public>,
    U: Never<Decrease, Size> + Layout<Public>,
    <U as Layout<Public>>::ByteLevel: FromLayout<<T as Layout<Public>>::ByteLevel,
      Variance,
      Alignment,
      Transparency,
      Stable>
{}