#![no_std]
#![allow(warnings)]

//! Typic helps you transmute fearlessly. It worries about the subtleties of
//! ***[soundness]*** and ***[safety]*** so you don't have to!
//!
//! Just import it and replace your `#[repr(...)]` attributes with `#[typic::repr(...)]`:
//! ```
//! // Import it!
//! use typic::{self, transmute::StableTransmuteInto, stability::StableABI};
//!
//! // Update your attributes!
//! #[typic::repr(C)]
//! #[derive(StableABI)]
//! pub struct Foo(pub u8, pub u16);
//!
//! // Transmute fearlessly!
//! let _ : Foo = 64u32.transmute_into(); // Alchemy achieved!
//! ```
//! ```compile_fail
//! # use typic::{self, TransmuteInto};
//! # #[typic::repr(C)]
//! # #[derive(StableABI)]
//! # struct Foo(pub u8, pub u16);
//! let _ : u32 = Foo(16, 12).transmute_into(); // Compile Error!
//! ```
//!
//! [soundness]: crate::transmute::unsafe_transmutation#when-is-a-transmutation-sound
//! [safety]: crate::transmute::safe_transmutation

#[doc(hidden)]
pub mod docs {
    pub mod prelude {
        pub use crate::stability::StableABI;
        pub use crate::transmute::{unsafe_transmute, StableTransmuteInto};
        use crate::typic;
        pub use core::mem;
        pub use core::num::NonZeroU8;

        #[typic::repr(C)]
        #[derive(Default, StableABI)]
        pub struct Padded(pub u8, pub u16, pub u8);

        #[typic::repr(C)]
        #[derive(Default, StableABI)]
        pub struct Packed(pub u16, pub u16, pub u16);

        #[typic::repr(C)]
        #[derive(Default, StableABI)]
        pub struct Constrained {
            wizz: i8,
            bang: u8,
        }

        impl Constrained {
            /// the sum of `wizz` and `bang` must be greater than or equal to zero.
            pub fn new(wizz: i8, bang: u8) -> Self {
                assert!((wizz as i16) / (bang as i16) >= 0);
                Constrained { wizz, bang }
            }

            pub fn something_dangerous(&self) {
                unsafe {
                    // do something that's only safe if `wizz + bang >= 0`
                }
            }
        }

        #[typic::repr(C)]
        #[derive(Default, StableABI)]
        pub struct Unconstrained {
            pub wizz: u8,
            pub bang: i8,
        }
    }
}

#[doc(hidden)]
#[deprecated(note = "TODO")]
pub enum TODO {}

#[doc(hidden)]
pub mod private {
    pub mod bytelevel;
    pub mod highlevel;
    pub mod layout;
    pub mod num;
    pub mod stability;
    pub mod target;
    pub mod transmute;
}

#[doc(hidden)]
pub use private::highlevel as internal;

/// Use `#[typic::repr(...)]` instead of `#[repr(...)]` on your type definitions.
#[doc(inline)]
pub use typic_derive::repr;

#[doc(inline)]
pub use private::stability;

pub mod transmute;

mod typic {
    pub use super::*;
}

/// Details about the layout of types.
///
/// [`SizeOf`]: crate::layout::SizeOf
/// [`zerocopy`]: https://crates.io/crates/zerocopy
/// [`AsBytes`]: https://docs.rs/zerocopy/0.2.*/zerocopy/trait.AsBytes.html
/// [`FromBytes`]: https://docs.rs/zerocopy/0.2.*/zerocopy/trait.FromBytes.html
/// [`Unaligned`]: https://docs.rs/zerocopy/0.2.*/zerocopy/trait.Unaligned.html
///
/// Useful for building your own abstractions over Typic. For instance, we can
/// use [`SizeOf`] to implement [`zerocopy`]'s [`FromBytes`], [`AsBytes`] and
/// [`Unaligned`] marker traits:
///
/// ```
/// use typic::{layout::{Layout, SizeOf}, transmute::TransmuteInto, transmute::TransmuteFrom};
/// use generic_array::{ArrayLength as Length, GenericArray as Array};
/// use typenum::U1;
///
/// /// Indicates `Self` can be produced from an
/// /// appropriately-sized array of arbitrarily
/// /// initialized bytes.
/// pub trait FromBytes {}
///
/// impl<T> FromBytes for T
/// where
///     T: Layout,
///     SizeOf<T>: Length<u8>,
///     T: TransmuteFrom<Array<u8, SizeOf<T>>>
/// {}
///
/// /// Indicates `Self` can be converted into an
/// /// appropriately-sized array of arbitrarily
/// /// initialized bytes.
/// pub trait AsBytes {}
///
/// impl<T> AsBytes for T
/// where
///     T: Layout,
///     SizeOf<T>: Length<u8>,
///     T: TransmuteInto<Array<u8, SizeOf<T>>>
/// {}
///
/// /// Indicates `Self` has no alignment requirement.
/// pub trait Unaligned {}
///
/// impl<T> Unaligned for T
/// where
///     T: Layout<Align=U1>,
/// {}
/// ```
pub mod layout {
    use crate::internal::{Private, Public};
    use crate::private::{layout, num};
    use generic_array::ArrayLength;

    /// Type-level information about type representation.
    pub trait Layout: layout::Layout<Public> {
        /// The size of `Self`.
        ///
        /// ```
        /// use typenum::*;
        /// use static_assertions::*;
        /// use typic::layout::Layout;
        ///
        /// assert_type_eq_all!(U4, <[u16; 2] as Layout>::Size);
        /// ```
        type Size: num::Unsigned + ArrayLength<u8>;

        /// The minimum alignment of `Self`.
        ///
        /// ```
        /// use typenum::*;
        /// use static_assertions::*;
        /// use typic::layout::Layout;
        ///
        /// assert_type_eq_all!(U2, <[u16; 2] as Layout>::Align);
        /// ```
        type Align: num::Unsigned;
    }

    impl<T> Layout for T
    where
        T: layout::Layout<Public>,
    {
        type Size = <T as layout::Layout<Public>>::Size;
        type Align = <T as layout::Layout<Public>>::Align;
    }

    /// Get the size of `T` (if `T: Layout`).
    ///
    /// ```
    /// use typenum::*;
    /// use static_assertions::*;
    /// use typic::layout::SizeOf;
    ///
    /// assert_type_eq_all!(U4, SizeOf<[u16; 2]>);
    /// ```
    pub type SizeOf<T> = <T as Layout>::Size;

    /// Get the minimum alignment of `T` (if `T: Layout`).
    ///
    /// ```
    /// use typenum::*;
    /// use static_assertions::*;
    /// use typic::layout::AlignOf;
    ///
    /// assert_type_eq_all!(U2, AlignOf<[u16; 2]>);
    /// ```
    pub type AlignOf<T> = <T as Layout>::Align;
}

/// Examples demonstrating typic's ability to express common abstractions.
pub mod extras {

    /// [Zerocopy](https://docs.rs/zerocopy)-style marker traits.
    pub mod zerocopy {
        use crate::layout::*;
        use crate::transmute::*;
        use typenum::U1;
        use generic_array::{ArrayLength as Length, GenericArray as Array};

        /// Indicates `Self` can be produced from an
        /// appropriately-sized array of arbitrarily
        /// initialized bytes.
        pub unsafe trait FromBytes<O: TransmuteOptions = ()>
        {}

        unsafe impl<T, O: TransmuteOptions> FromBytes<O> for T
        where
            T: Layout + TransmuteFrom<Array<u8, SizeOf<T>>, O>
        {}


        /// Indicates `Self` can be converted into an
        /// appropriately-sized array of arbitrarily
        /// initialized bytes.
        pub unsafe trait AsBytes<O: TransmuteOptions = ()> {}

        unsafe impl<T, O: TransmuteOptions> AsBytes<O> for T
        where
            T: Layout + TransmuteInto<Array<u8, SizeOf<T>>, O>
        {}


        /// Indicates `Self` has no alignment requirement.
        pub trait Unaligned {}

        impl<T> Unaligned for T
        where
            T: Layout<Align=U1>,
        {}
    }

    /// [Bytemuck](https://docs.rs/bytemuck)-style casting functions.
    pub mod bytemuck {
        use crate::transmute::*;
        use core::mem::{align_of, size_of, size_of_val};

        /// Try to convert a `&T` into `&U`.
        ///
        /// This produces `None` if the referent isn't appropriately
        /// aligned, as required by the destination type.
        ///
        /// Like [`bytemuck::try_cast_ref`], except that invariant
        /// that `T` and `U` have the same size is statically enforced.
        ///
        /// [`bytemuck::try_cast_ref`]: https://docs.rs/bytemuck/1.2.0/bytemuck/fn.try_cast_ref.html
        pub fn try_cast_ref<'t, 'u, T, U>(src: &'t T) -> Option<&'u U>
        where
            &'t T: UnsafeTransmuteInto<&'u U, neglect::Alignment>,
        {
            if align_of::<U>() > align_of::<T>() && (src as *const T as usize) % align_of::<U>() != 0 {
                None
            } else {
                // Sound, because we dynamically enforce the alignment
                // requirement, whose static check we chose to neglect.
                Some(unsafe { src.unsafe_transmute_into() })
            }
        }

        use core::slice;
        use generic_array::{ArrayLength as Length, GenericArray as Array};
        use crate::layout::*;

        /// Try to convert a `&T` into `&U`.
        ///
        /// This produces `None` if the referent isn't appropriately
        /// aligned, as required by the destination type.
        ///
        /// Like [`bytemuck::try_cast_slice`], except that invariant
        /// that `T` and `U` have the same size is statically enforced.
        ///
        /// If const generics were stable, the trait bound would
        /// be instead written as just:
        /// ```ignore
        /// &'t [T; size_of::<U>()]:
        ///     TransmuteInto<&'u [U; size_of::<T>()]>
        /// ```
        ///
        /// [`bytemuck::try_cast_slice`]: https://docs.rs/bytemuck/1.2.0/bytemuck/fn.try_cast_slice.html
        pub fn try_cast_slice<'t, 'u, T, U>(src: &'t [T]) -> Option<&'u [U]>
        where
            &'t Array<T, SizeOf<U>>: TransmuteInto<&'u Array<U, SizeOf<T>>>,

            T: Layout,
            U: Layout,
            SizeOf<T>: 'u + Length<U>,
            SizeOf<U>: 't + Length<T>,
        {
            if align_of::<U>() > align_of::<T>() && (src.as_ptr() as usize) % align_of::<U>() != 0 {
                None
            } else {
                let len = size_of_val(src).checked_div(size_of::<U>()).unwrap_or(0);
                Some(unsafe {
                  slice::from_raw_parts(src.as_ptr() as *const U, len)
                })
            }
        }
    }
}