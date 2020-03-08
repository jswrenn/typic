#![no_std]
#![allow(warnings)]

//! Typic helps you transmute fearlessly. It worries about the subtleties of
//! ***[soundness]*** and ***[safety]*** so you don't have to!
//!
//! Just import it and replace your `#[repr(...)]` attributes with `#[typic::repr(...)]`:
//! ```
//! // Import it!
//! use typic::{self, StableTransmuteInto, StableABI};
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
//! [soundness]: crate::sound#when-is-a-transmutation-sound
//! [safety]: crate::safe
//!
//! ## Three Types of Transmutation
//!
//! ### Unsound Transmutation
//! [`transmute`]: core::mem::transmute
//! [`transmute_copy`]: core::mem::transmute_copy
//!
//! The [`transmute`] and [`transmute_copy`] intrinsics
//! allow for the ***unsafe*** and ***unsound*** transmutation between any `T`
//! and `U`.
//!
//! These intrinsics are deeply unsafe. The Rust compiler will accept uses of
//! these intrinsics even when `T` and `U` do not have well-defined layouts.
//! ***Always use a [safe transmutation](#safe-transmutation) method instead,
//! if possible.*** If you are unable to use a safe transmutation method,
//! ***you may be relying on undefined compiler behavior***.
//!
//! ### Sound Transmutation
//! [`transmute_sound`]: crate::transmute_sound
//!
//! The [`transmute_sound`] function allows for the ***unsafe*** transmutation
//! between `T` and `U`, when merely transmuting from `T` to `U` will not cause
//! undefined behavior. For the key rules that govern when `T` is soundly
//! convertible to `U`, see ***[When is a transmutation sound?][soundness]***.
//!
//! This operation is `unsafe`, as it will bypass any user-defined validity
//! restrictions that `U` places on its fields and enforces with its
//! constructors and methods.
//!
//! ***Always use a [safe transmutation](#safe-transmutation) method instead, if
//! possible.*** If you are unable to use a safe transmutation method, you may
//! be violating library invariants.
//!
//! ### Safe Transmutation
//! [safe transmutation]: #safe-transmutation
//! [`TransmuteInto<U>`]: crate::TransmuteInto
//!
//! The [`TransmuteInto<U>`] trait is implemented for a type `T` if:
//! 1. [`T` is ***soundly*** transmutable into `U`][soundness], and
//! 2. [`T` is ***safely*** transmutable into `U`][safety].
//!
//! If you are unable to use [`TransmuteInto<U>`], you may be attempting a
//! transmutation that is relying unspecified behavior.
#[doc(hidden)]
pub mod docs {
    pub mod prelude {
        use crate::typic;
        pub use crate::StableABI;
        pub use crate::{transmute_sound, StableTransmuteInto};
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
    pub mod target;
    pub mod transmute;
}

#[doc(hidden)]
pub use private::highlevel as internal;

#[doc(inline)]
pub use private::transmute::{transmute_safe, transmute_sound, TransmuteFrom, TransmuteInto, StableTransmuteInto};

/// Use `#[typic::repr(...)]` instead of `#[repr(...)]` on your type definitions.
#[doc(inline)]
pub use typic_derive::{repr, StableABI};

mod typic {
    pub use super::*;
}

pub mod neglect {
    #[doc(inline)]
    pub use crate::private::transmute::neglect::*;
}

pub trait StableABI {}

/// Guidance and tools for ***safe*** transmutation.
///
/// A [sound transmutation] is safe only if the resulting value cannot possibly
/// violate library-enforced invariants. Typic assumes that all non-zero-sized
/// fields with any visibility besides `pub` could have library-enforced
/// invariants.
///
/// [sound transmutation]: crate#sound-transmutation
/// [soundness]: crate::sound#when-is-a-transmutation-sound
/// [`TransmuteInto`]: crate::TransmuteInto
/// [`transmute_sound`]: crate::transmute_sound
///
/// ## Why is safety different than soundness?
/// Consider the type `Constrained`, which enforces a validity constraint on its
/// fields, and the type `Unconstrained` (which has no internal validity
/// constraints):
///
/// ```
/// # use typic::docs::prelude::*;
/// #[typic::repr(C)]
/// #[derive(StableABI)]
/// pub struct Constrained {
///     wizz: i8,
///     bang: u8,
/// }
///
/// impl Constrained {
///     /// the sum of `wizz` and `bang` must be greater than or equal to zero.
///     pub fn new(wizz: i8, bang: u8) -> Self {
///         assert!((wizz as i16) / (bang as i16) >= 0);
///         Constrained { wizz, bang }
///     }
///
///     pub fn something_dangerous(&self) {
///         unsafe {
///             // do something that's only safe if `wizz + bang >= 0`
///         }
///     }
/// }
///
/// #[typic::repr(C)]
/// #[derive(StableABI)]
/// pub struct Unconstrained {
///     pub wizz: u8,
///     pub bang: i8,
/// }
/// ```
///
/// It is [sound][soundness] to transmute an instance of `Unconstrained` into
/// `Constrained`:
/// ```
/// use typic::docs::prelude::*;
/// use typic::neglect;
/// let _ : Constrained  = unsafe { transmute_sound::<_, _, neglect::Transparency>(Unconstrained::default()) };
/// ```
/// ...but it is **not** safe! The [`transmute_sound`] function creates an
/// instance of `Bar` _without_ calling its `new` constructor, thereby bypassing
/// the safety check which ensures `something_dangerous` does not violate Rust's
/// memory model. The compiler will reject our program if we try to safely
/// transmute `Unconstrained` to `Constrained`:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let unconstrained = Unconstrained::default();
/// let _ : Constrained  = unconstrained.transmute_into();
/// ```
///
/// Or, ***automatically***, by marking the fields `pub`:
/// ```
/// # use typic::docs::prelude::*;
/// #[typic::repr(C)]
/// #[derive(StableABI)]
/// pub struct Unconstrained {
///     pub wizz: u8,
///     pub bang: i8,
/// }
///
/// let _ : Unconstrained = u16::default().transmute_into();
/// ```
///
/// If the fields are marked `pub`, the type cannot possibly rely on any
/// internal validity requirements, as users of the type are free to manipulate
/// its fields direclty via the `.` operator.
///
/// ## Safely transmuting references
/// When safely transmuting owned values, all non-padding bytes in the source
/// type must correspond to `pub` bytes in the destination type:
/// ```
/// # use typic::docs::prelude::*;
/// let _ : Unconstrained = Constrained::default().transmute_into();
/// ```
/// The visibility (or lack thereof) of bytes in the source type does not
/// affect safety.
///
/// When safely transmuting references, each corresponding byte in the source
/// and destination types must have the _same_ visibility. Without this
/// restriction, you could inadvertently violate library invariants of a type
/// by transmuting and mutating a mutable reference to it:
///
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let mut x = Constrained::default();
///
/// {
///     let y : &mut Unconstrained = (&mut x).transmute_into();
///                                        // ^^^^^^^^^^^^^^
///                                        // Compile Error!
///     let z : u8 = -100i8.transmute_into();
///     y.wizz = z;
/// }
///
/// // Ack! `x.wizz + x.bang` is now -100!
/// // This violates the safety invariant of `something_dangerous`!
/// x.something_dangerous();
/// ```
pub mod safe {
    #[doc(inline)]
    pub use crate::{transmute_safe, TransmuteFrom, TransmuteInto, StableTransmuteInto};
}

/// Guidance and tools for ***sound*** transmutation.
///
/// A transmutation is ***sound*** if the mere act of transmutation is
/// guaranteed to not violate Rust's memory model.
///
/// [`transmute_sound`]: crate::transmute_sound
/// [`TransmuteInto<U>`]: crate::TransmuteInto
///
/// ## When is a transmutation sound?
/// [`NonZeroU8`]: core::num::NonZeroU8
///
/// A transmutation is only sound if it occurs between types with [well-defined
/// representations](#well-defined-representation), and does not violate Rust's
/// memory model. See [*Transmutations Between Owned Values*][transmute-owned],
/// and [*Transmutations Between References*][transmute-references]. These rules
/// are automatically enforced by [`transmute_sound`] and [`TransmuteInto<U>`].
///
/// ### Well-Defined Representation
/// [`u8`]: core::u8
/// [`f32`]: core::f32
///
/// Transmutation is ***always unsound*** if it occurs between types with
/// unspecified representations. Most of Rust's primitive types have specified
/// representations. That is, the layout characteristics of [`u8`], [`f32`] and
/// others are guaranteed to be stable across compiler versions.
///
/// In contrast, most `struct` and `enum` types defined without an explicit
/// `#[repr(C)]` or `#[repr(transparent)]` attribute do ***not*** have
/// well-specified layout characteristics.
///
/// To ensure that types you've define are soundly transmutable, you usually
/// must mark them with the `#[repr(C)]` attribute.
///
/// ### Transmuting Owned Values
/// [transmute-owned]: #transmuting-owned-values
///
/// Transmutations involving owned values must adhere to two rules to be sound.
/// They must:
///  * [preserve or broaden the bit validity][owned-validity], and
///  * [preserve or shrink the size][owned-size].
///
/// #### Preserve or Broaden Bit Validity
/// [owned-validity]: #preserve-or-broaden-bit-validity
///
/// For each _i<sup>th</sup>_ of the destination type, all possible
/// instantiations of the _i<sup>th</sup>_ byte of the source type must be a
/// bit-valid instance of the _i<sup>th</sup>_ byte of the destination type.
///
/// For example, we are permitted us to transmute a [`NonZeroU8`] into a [`u8`]:
/// ```rust
/// # use typic::docs::prelude::*;
/// let _ : u8 = NonZeroU8::new(1).unwrap().transmute_into();
/// ```
/// ...because all possible instances of [`NonZeroU8`] are also valid instances
/// of [`u8`]. However, transmuting a [`u8`] into a [`NonZeroU8`] is forbidden:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let _ : NonZeroU8 = u8::default().transmute_into(); // Compile Error!
/// ```
/// ...because not all instances of [`u8`] are valid instances of [`NonZeroU8`].
///
/// Another example: While laying out certain types, rust may insert padding
/// bytes between the layouts of fields. In the below example `Padded` has two
/// padding bytes, while `Packed` has none:
/// ```rust
/// # use typic::docs::prelude::*;
/// #[typic::repr(C)]
/// #[derive(Default)]
/// struct Padded(pub u8, pub u16, pub u8);
///
/// #[typic::repr(C)]
/// #[derive(Default)]
/// struct Packed(pub u16, pub u16, pub u16);
///
/// assert_eq!(mem::size_of::<Packed>(), mem::size_of::<Padded>());
/// ```
///
/// We may safely transmute from `Packed` to `Padded`:
/// ```rust
/// # use typic::docs::prelude::*;
/// let _ : Padded = Packed::default().transmute_into();
/// ```
/// ...but not from `Padded` to `Packed`:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let _ : Packed = Padded::default().transmute_into(); // Compile Error!
/// ```
/// ...because doing so would expose two uninitialized padding bytes in `Padded`
/// as if they were initialized bytes in `Packed`.
///
/// #### Preserve or Shrink Size
/// [owned-size]: #preserve-or-shrink-size
///
/// It's completely safe to transmute into a type with fewer bytes than the
/// destination type; e.g.:
/// ```rust
/// # use typic::docs::prelude::*;
/// let _ : u8 = u64::default().transmute_into();
/// ```
/// This transmute truncates away the final three bytes of the `u64` value.
///
/// A value may ***not*** be transmuted into a type of greater size:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let _ : u64 = u8::default().transmute_into(); // Compile Error!
/// ```
///
/// ### Transmuting References
/// [transmute-references]: #transmuting-references
///
/// The [restrictions above that to transmuting owned values][transmute-owned],
/// also apply to transmuting references. However, references carry a few
/// additional restrictions. A [sound transmutation](#sound-transmutation) must:
///  - [preserve or relax alignment][reference-alignment],
///  - [preserve or shrink lifetimes][reference-lifetimes],
///  - [preserve or shrink mutability][reference-mutability], and
///  - [preserve validity][reference-validity].
///
/// #### Preserve or Relax Alignment
/// [reference-alignment]: #preserve-or-relax-alignment
///
/// You may transmute a reference into reference of more relaxed alignment:
/// ```rust
/// # use typic::docs::prelude::*;
/// let _: &[u8; 0] = (&[0u16; 0]).transmute_into();
/// ```
///
/// However, you may **not** transmute a reference into a reference of stricter
/// alignment:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let _: &[u16; 0] = (&[0u8; 0]).transmute_into(); // Compile Error!
/// ```
///
/// #### Preserve or Shrink Lifetimes
/// [reference-lifetimes]: #preserve-or-shrink-lifetimes
///
/// You may transmute a reference into reference of lesser lifetime:
/// ```rust
/// # use typic::docs::prelude::*;
/// fn shrink<'a>() -> &'a u8 {
///     static long : &'static u8 =  &16;
///     long
/// }
/// ```
///
/// However, you may **not** transmute a reference into a reference of greater
/// lifetime:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// fn extend<'a>(short: &'a u8) -> &'static u8 {
///     static long : &'static u8 =  &16;
///     short.transmute_into()
/// }
/// ```
///
/// #### Preserve or Shrink Mutability
/// [reference-mutability]: #preserve-or-shrink-mutability
///
/// You may preserve or decrease the mutability of a reference through
/// transmutation:
/// ```rust
/// # use typic::docs::prelude::*;
/// let _: &u8 = (&42u8).transmute_into();
/// let _: &u8 = (&mut 42u8).transmute_into();
/// ```
///
/// However, you may **not** transmute an immutable reference into a mutable
/// reference:
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let _: &mut u8 = (&42u8).transmute_into(); // Compile Error!
/// ```
///
/// #### Preserve Validity
/// [reference-validity]: #preserve-validity
///
/// Unlike transmutations of owned values, the transmutation of a reference may
/// also not expand the bit-validity of the referenced type. For instance:
///
/// ```compile_fail
/// # use typic::docs::prelude::*;
/// let mut x = NonZeroU8::new(42).unwrap();
/// {
///     let y : &mut u8 = (&mut x).transmute_into(); // Compile Error!
///     *y = 0;
/// }
///
/// let z : NonZeroU8 = x;
/// ```
/// If this example did not produce a compile error, the value of `z` would not
/// be a bit-valid instance of its type.
pub mod sound {
    pub use crate::transmute_sound;
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
/// use typic::{layout::{Layout, SizeOf}, TransmuteInto, TransmuteFrom};
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
    use crate::private::{layout, num};
    use crate::internal::{Public, Private};

    /// Type-level information about type representation.
    pub trait Layout {
        /// The size of `Self`.
        ///
        /// ```
        /// use typenum::*;
        /// use static_assertions::*;
        /// use typic::layout::Layout;
        ///
        /// assert_type_eq_all!(U4, <[u16; 2] as Layout>::Size);
        /// ```
        type Size: num::Unsigned;

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
