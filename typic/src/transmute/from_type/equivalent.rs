use super::from_layout;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait Equivalent<T> {}

unsafe impl<T, U> Equivalent<U> for T where U: from_layout::Equivalent<T> {}
