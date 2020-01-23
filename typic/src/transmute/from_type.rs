use super::from_layout;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait FromType<T> {}

unsafe impl<T, U> FromType<U> for T where U: from_layout::FromLayout<T> {}
