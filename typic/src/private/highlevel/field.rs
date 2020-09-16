use crate::private::num::Min;
use core::marker::PhantomData;

pub struct Public;
pub struct Private;

impl Min<Public> for Public {
    type Output = Public;
    fn min(self, rhs: Public) -> Self::Output {
        Public
    }
}

impl Min<Private> for Private {
    type Output = Private;
    fn min(self, rhs: Private) -> Self::Output {
        Private
    }
}

impl Min<Public> for Private {
    type Output = Private;
    fn min(self, rhs: Public) -> Self::Output {
        Private
    }
}

impl Min<Private> for Public {
    type Output = Private;
    fn min(self, rhs: Private) -> Self::Output {
        Private
    }
}

pub struct Field<Vis, Type>(PhantomData<(Vis, Type)>);
