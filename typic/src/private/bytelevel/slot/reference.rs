//! `& T` and `&mut T`

use core::marker::PhantomData;

pub struct Shared;
pub struct Unique;

pub struct Reference<'a, K, T>(PhantomData<(K, &'a T)>);

/// A unique reference to a type `T` with lifetime `'a`.
pub type UniqueRef<'a, T> = Reference<'a, Unique, T>;

/// A shared reference to a type `T` with lifetime `'a`.
pub type SharedRef<'a, T> = Reference<'a, Shared, T>;
