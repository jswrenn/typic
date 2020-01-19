//! `& T` and `&mut T`

use core::marker::PhantomData;

/// A unique reference to a type `T` with lifetime `'a`.
pub struct UniqueRef<'a, T>(PhantomData<&'a T>);

/// A shared reference to a type `T` with lifetime `'a`.
pub struct SharedRef<'a, T>(PhantomData<&'a T>);
