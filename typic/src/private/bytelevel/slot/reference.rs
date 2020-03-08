//! `& T` and `&mut T`

use core::marker::PhantomData;

pub struct Shared;
pub struct Unique;

pub struct Reference<'a, Visibility, K, T>(PhantomData<(Visibility, K, &'a T)>);

/// A unique reference to a type `T` with lifetime `'a`.
pub type UniqueRef<'a, Visibility, T> = Reference<'a, Visibility, Unique, T>;

/// A shared reference to a type `T` with lifetime `'a`.
pub type SharedRef<'a, Visibility, T> = Reference<'a, Visibility, Shared, T>;
