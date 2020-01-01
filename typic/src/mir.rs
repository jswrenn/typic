use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::num::NonZeroU8;
use core::ops::Add;
use typenum::*;

pub trait Representation {}

impl Representation for u8 {}

impl Size for u8 {
    type Size = U1;
}

impl Representation for MaybeUninit<u8> {}

impl Size for Uninit {
    type Size = U1;
}

impl Representation for NonZeroU8 {}

impl Size for NonZeroU8 {
    type Size = U1;
}

pub trait Size {
    type Size: Unsigned;
}

pub use coproduct::Coproduct;
pub use product::Product;

pub type Init = u8;
pub type Uninit = MaybeUninit<u8>;
pub type NonZero = NonZeroU8;

impl<'t, T> Size for &'t T {
    #[cfg(target_pointer_width = "64")]
    type Size = U8;
}

impl<'t, T> Size for &'t mut T {
    #[cfg(target_pointer_width = "64")]
    type Size = U8;
}

pub mod product {
    use super::Size;
    use core::marker::PhantomData;
    use core::ops::*;
    use typenum::*;

    pub trait Product: super::Representation {}

    pub struct Nil;

    impl Product for Nil {}
    impl super::Representation for Nil {}
    impl super::Size for Nil {
        type Size = U0;
    }

    pub struct Cons<H, T>(PhantomData<(H, T)>)
    where
        H: super::Representation,
        T: Product;

    impl<H, T> Product for Cons<H, T>
    where
        H: super::Representation,
        T: Product,
    {
    }

    impl<H, T> super::Size for Cons<H, T>
    where
        H: super::Representation + Size,
        T: Product + Size,
        <H as Size>::Size: Add<<T as Size>::Size>,
        Sum<<H as Size>::Size, <T as Size>::Size>: Unsigned,
    {
        type Size = Sum<<H as Size>::Size, <T as Size>::Size>;
    }

    impl<H, T> super::Representation for Cons<H, T>
    where
        H: super::Representation,
        T: Product,
    {
    }
}

pub mod coproduct {
    use super::Size;
    use core::marker::PhantomData;
    use core::ops::*;
    use typenum::*;

    pub trait Coproduct: super::Representation {}

    pub struct Nil<H>(PhantomData<H>)
    where
        H: super::Representation;

    impl<H> Coproduct for Nil<H> where H: super::Representation {}
    impl<H> super::Representation for Nil<H> where H: super::Representation {}
    impl<H> super::Size for Nil<H>
    where
        H: super::Representation,
    {
        type Size = U0;
    }

    pub struct Cons<H, T>(PhantomData<(H, T)>)
    where
        H: super::Representation,
        T: Coproduct;

    impl<H, T> Coproduct for Cons<H, T>
    where
        H: super::Representation,
        T: Coproduct,
    {
    }

    impl<H, T> super::Size for Cons<H, T>
    where
        H: super::Representation + Size,
        T: Coproduct + Size,
        <H as Size>::Size: Add<<T as Size>::Size>,
        Sum<<H as Size>::Size, <T as Size>::Size>: Unsigned,
    {
        type Size = Sum<<H as Size>::Size, <T as Size>::Size>;
    }

    impl<H, T> super::Representation for Cons<H, T>
    where
        H: super::Representation,
        T: Coproduct,
    {
    }
}

impl<RHS> Add<RHS> for product::Nil
where
    RHS: product::Product,
{
    type Output = RHS;

    fn add(self, rhs: RHS) -> RHS {
        unimplemented!()
    }
}

impl<H, T, RHS> Add<RHS> for product::Cons<H, T>
where
    H: Representation,
    T: product::Product,
    T: Add<RHS>,
    RHS: product::Product,
    <T as Add<RHS>>::Output: product::Product,
{
    type Output = product::Cons<H, <T as Add<RHS>>::Output>;

    fn add(self, rhs: RHS) -> Self::Output {
        unimplemented!()
    }
}

impl<S, const T: &'static [u8]> Representation for crate::hir::Discriminant<S, { T }> {}

impl<S: crate::hir_into_mir::Layout, const T: &'static [u8]> Size
    for crate::hir::Discriminant<S, { T }>
where
    <S as crate::hir_into_mir::Layout>::Representation: Size,
{
    type Size = <<S as crate::hir_into_mir::Layout>::Representation as Size>::Size;
}

impl<S: crate::hir_into_mir::Layout, MinAlign, const T: &'static [u8]>
    crate::hir_into_mir::Align<MinAlign> for crate::hir::Discriminant<S, { T }>
where
    <S as crate::hir_into_mir::Layout>::Representation: Size,
{
    type Output = <<S as crate::hir_into_mir::Layout>::Representation as Size>::Size;
}

impl<S, MinAlign, const T: &'static [u8]> crate::hir_into_mir::MinSize<MinAlign>
    for crate::hir::Discriminant<S, { T }>
{
    type Output = U1;
}

impl<S, MinAlign, MinSize, Offset, const T: &'static [u8]>
    crate::hir_into_mir::IntoMIR<MinAlign, MinSize, Offset> for crate::hir::Discriminant<S, { T }>
where
    MinAlign: Unsigned,
    MinSize: Unsigned,
    Offset: Unsigned,
    S: crate::hir_into_mir::Layout,
{
    type Output = product::Cons<Self, product::Nil>;
}
