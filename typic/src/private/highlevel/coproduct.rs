use core::marker::PhantomData;

pub trait Coproduct {}

pub struct Nil;

impl Coproduct for Nil {}

pub struct Cons<H, T>(PhantomData<(H, T)>);

impl<H, T> Coproduct for Cons<H, T> {}
