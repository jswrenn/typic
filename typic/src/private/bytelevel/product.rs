use core::marker::PhantomData;

pub trait Product {}

pub struct Nil;

impl Product for Nil {}

pub struct Cons<H, T>(PhantomData<(H, T)>);

impl<H, T> Product for Cons<H, T> {}
