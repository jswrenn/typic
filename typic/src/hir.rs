use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::Add;

pub struct Discriminant<S, const T: &'static [u8]>(PhantomData<S>);

impl<S, const T: &'static [u8]> Representation for Discriminant<S, { T }> {}

impl<S, const T: &'static [u8]> Type for Discriminant<S, { T }> {
    type Padding = padding::Padded;
    type Representation = Self;
}

pub mod padding {
    /// A marker indicating that a compound type is `#[repr(packed)]`
    pub struct Packed;

    /// A marker indicating that a compound type is not `#[repr(packed)]`.
    pub struct Padded;

    /// A trait defining the set of possible padding modes.
    pub trait Padding {}

    impl Padding for Packed {}

    impl Padding for Padded {}
}

/// A generic representation of a type.
pub trait Type {
    /// The padding mode of the type.
    type Padding: padding::Padding;

    /// An abstract representation of the type's structure.
    type Representation: Representation;
}

pub trait Candidate {
    type Candidate;
}

pub trait Representation {}

pub enum Uninhabited {}
impl Representation for Uninhabited {}

pub mod product {
    use core::marker::PhantomData;

    pub trait Product: super::Representation {}

    pub struct Nil;

    impl Product for Nil {}
    impl super::Representation for Nil {}

    pub struct Cons<H, T>(PhantomData<(H, T)>)
    where
        T: Product;

    impl<H, T> Product for Cons<H, T> where T: Product {}

    impl<H, T> super::Representation for Cons<H, T> where T: Product {}
}

pub mod coproduct {
    use core::marker::PhantomData;

    pub trait Coproduct: super::Representation {}

    pub struct Nil<H>(PhantomData<H>);

    impl<H> Coproduct for Nil<H> {}
    impl<H> super::Representation for Nil<H> {}

    pub struct Cons<H, T>(PhantomData<(H, T)>)
    where
        T: Coproduct;

    impl<H, T> Coproduct for Cons<H, T> where T: Coproduct {}

    impl<H, T> super::Representation for Cons<H, T> where T: Coproduct {}
}
