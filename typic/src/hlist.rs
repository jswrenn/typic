use core::marker::PhantomData;
use core::ops::Add;

pub struct HCons<H, T> {
    data: PhantomData<(H, T)>,
}

impl<H, T> HCons<H, T> {
    fn new() -> HCons<H, T> {
        HCons { data: PhantomData }
    }
}

pub struct HNil {}

pub trait HList {}

impl HList for HNil {}

impl<H, T> HList for HCons<H, T> {}

impl<RHS> Add<RHS> for HNil
where
    RHS: HList,
{
    type Output = RHS;

    fn add(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for HCons<H, T>
where
    T: Add<RHS>,
    RHS: HList,
{
    type Output = HCons<H, <T as Add<RHS>>::Output>;

    fn add(self, _rhs: RHS) -> Self::Output {
        HCons::new()
    }
}

pub trait IntoReverse {
    type Output;
}

impl IntoReverse for HNil {
    type Output = HNil;
}

impl<H, Tail> IntoReverse for HCons<H, Tail>
where
    Tail: IntoReverse,
    <Tail as IntoReverse>::Output: Add<HCons<H, HNil>>,
{
    type Output = <<Tail as IntoReverse>::Output as Add<HCons<H, HNil>>>::Output;
}
