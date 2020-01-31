use super::from_layout;

/// A marker trait implemented if every instance of `T` is transmutable into
/// an instance of `Self`.
pub unsafe trait Subsumes<T> {}

unsafe impl<T, U> Subsumes<U> for T where U: from_layout::Subsumes<T> {}
