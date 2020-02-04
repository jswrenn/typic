//! [T; N]

use core::marker::PhantomData;

/// A unique reference to a type `T` with lifetime `'a`.
pub struct Array<T, N>(PhantomData<(T, N)>);
