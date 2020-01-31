use core::mem;

#[rustfmt::skip]
mod from_type;

#[rustfmt::skip]
mod from_layout;

/// Transmute `Self` into `U`, if all possible instantiations of `Self` are
/// valid instantiations of `U`.
pub unsafe trait Transmute<U> {
    #[inline(always)]
    fn transmute(self) -> U
    where
        Self: Sized,
    {
        unsafe {
            let to = mem::transmute_copy(&self);
            mem::forget(self);
            to
        }
    }
}

unsafe impl<T, U> Transmute<U> for T where U: from_type::Subsumes<T> {}
