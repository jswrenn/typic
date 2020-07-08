//! Guidance and tools for ***sound*** transmutation.
//!
//! A transmutation is ***sound*** if the mere act of transmutation is
//! guaranteed to not violate Rust's memory model.
//!
//! [`unsafe_transmute`]: crate::unsafe_transmute
//! [`TransmuteInto<U>`]: crate::TransmuteInto
//!
//! ## When is a transmutation sound?
//! [`NonZeroU8`]: core::num::NonZeroU8
//!
//! A transmutation is only sound if it occurs between types with [well-defined
//! representations](#well-defined-representation), and does not violate Rust's
//! memory model. See [*Transmutations Between Owned Values*][transmute-owned],
//! and [*Transmutations Between References*][transmute-references]. These rules
//! are automatically enforced by [`unsafe_transmute`] and [`TransmuteInto<U>`].
//!
//! ### Well-Defined Representation
//! [`u8`]: core::u8
//! [`f32`]: core::f32
//!
//! Transmutation is ***always unsound*** if it occurs between types with
//! unspecified representations. Most of Rust's primitive types have specified
//! representations. That is, the layout characteristics of [`u8`], [`f32`] and
//! others are guaranteed to be stable across compiler versions.
//!
//! In contrast, most `struct` and `enum` types defined without an explicit
//! `#[repr(C)]` or `#[repr(transparent)]` attribute do ***not*** have
//! well-specified layout characteristics.
//!
//! To ensure that types you've define are soundly transmutable, you usually
//! must mark them with the `#[repr(C)]` attribute.
//!
//! ### Transmuting Owned Values
//! [transmute-owned]: #transmuting-owned-values
//!
//! Transmutations involving owned values must adhere to two rules to be sound.
//! They must:
//!  * [preserve or broaden the bit validity][owned-validity], and
//!  * [preserve or shrink the size][owned-size].
//!
//! #### Preserve or Broaden Bit Validity
//! [owned-validity]: #preserve-or-broaden-bit-validity
//!
//! For each _i<sup>th</sup>_ of the destination type, all possible
//! instantiations of the _i<sup>th</sup>_ byte of the source type must be a
//! bit-valid instance of the _i<sup>th</sup>_ byte of the destination type.
//!
//! For example, we are permitted us to transmute a [`NonZeroU8`] into a [`u8`]:
//! ```rust
//! # use typic::docs::prelude::*;
//! let _ : u8 = NonZeroU8::new(1).unwrap().transmute_into();
//! ```
//! ...because all possible instances of [`NonZeroU8`] are also valid instances
//! of [`u8`]. However, transmuting a [`u8`] into a [`NonZeroU8`] is forbidden:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let _ : NonZeroU8 = u8::default().transmute_into(); // Compile Error!
//! ```
//! ...because not all instances of [`u8`] are valid instances of [`NonZeroU8`].
//!
//! Another example: While laying out certain types, rust may insert padding
//! bytes between the layouts of fields. In the below example `Padded` has two
//! padding bytes, while `Packed` has none:
//! ```rust
//! # use typic::docs::prelude::*;
//! #[typic::repr(C)]
//! #[derive(Default, StableABI)]
//! struct Padded(pub u8, pub u16, pub u8);
//!
//! #[typic::repr(C)]
//! #[derive(Default, StableABI)]
//! struct Packed(pub u16, pub u16, pub u16);
//!
//! assert_eq!(mem::size_of::<Packed>(), mem::size_of::<Padded>());
//! ```
//!
//! We may safely transmute from `Packed` to `Padded`:
//! ```rust
//! # use typic::docs::prelude::*;
//! let _ : Padded = Packed::default().transmute_into();
//! ```
//! ...but not from `Padded` to `Packed`:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let _ : Packed = Padded::default().transmute_into(); // Compile Error!
//! ```
//! ...because doing so would expose two uninitialized padding bytes in `Padded`
//! as if they were initialized bytes in `Packed`.
//!
//! #### Preserve or Shrink Size
//! [owned-size]: #preserve-or-shrink-size
//!
//! It's completely safe to transmute into a type with fewer bytes than the
//! destination type; e.g.:
//! ```rust
//! # use typic::docs::prelude::*;
//! let _ : u8 = u64::default().transmute_into();
//! ```
//! This transmute truncates away the final three bytes of the `u64` value.
//!
//! A value may ***not*** be transmuted into a type of greater size:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let _ : u64 = u8::default().transmute_into(); // Compile Error!
//! ```
//!
//! ### Transmuting References
//! [transmute-references]: #transmuting-references
//!
//! The [restrictions above that to transmuting owned values][transmute-owned],
//! also apply to transmuting references. However, references carry a few
//! additional restrictions. A [sound transmutation](#sound-transmutation) must:
//!  - [preserve or relax alignment][reference-alignment],
//!  - [preserve or shrink lifetimes][reference-lifetimes],
//!  - [preserve or shrink mutability][reference-mutability], and
//!  - [preserve validity][reference-validity].
//!
//! #### Preserve or Relax Alignment
//! [reference-alignment]: #preserve-or-relax-alignment
//!
//! You may transmute a reference into reference of more relaxed alignment:
//! ```rust
//! # use typic::docs::prelude::*;
//! let _: &[u8; 0] = (&[0u16; 0]).transmute_into();
//! ```
//!
//! However, you may **not** transmute a reference into a reference of stricter
//! alignment:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let _: &[u16; 0] = (&[0u8; 0]).transmute_into(); // Compile Error!
//! ```
//!
//! #### Preserve or Shrink Lifetimes
//! [reference-lifetimes]: #preserve-or-shrink-lifetimes
//!
//! You may transmute a reference into reference of lesser lifetime:
//! ```rust
//! # use typic::docs::prelude::*;
//! fn shrink<'a>() -> &'a u8 {
//!     static long : &'static u8 =  &16;
//!     long
//! }
//! ```
//!
//! However, you may **not** transmute a reference into a reference of greater
//! lifetime:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! fn extend<'a>(short: &'a u8) -> &'static u8 {
//!     static long : &'static u8 =  &16;
//!     short.transmute_into()
//! }
//! ```
//!
//! #### Preserve or Shrink Mutability
//! [reference-mutability]: #preserve-or-shrink-mutability
//!
//! You may preserve or decrease the mutability of a reference through
//! transmutation:
//! ```rust
//! # use typic::docs::prelude::*;
//! let _: &u8 = (&42u8).transmute_into();
//! let _: &u8 = (&mut 42u8).transmute_into();
//! ```
//!
//! However, you may **not** transmute an immutable reference into a mutable
//! reference:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let _: &mut u8 = (&42u8).transmute_into(); // Compile Error!
//! ```
//!
//! #### Preserve Validity
//! [reference-validity]: #preserve-validity
//!
//! Unlike transmutations of owned values, the transmutation of a reference may
//! also not expand the bit-validity of the referenced type. For instance:
//!
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let mut x = NonZeroU8::new(42).unwrap();
//! {
//!     let y : &mut u8 = (&mut x).transmute_into(); // Compile Error!
//!     *y = 0;
//! }
//!
//! let z : NonZeroU8 = x;
//! ```
//! If this example did not produce a compile error, the value of `z` would not
//! be a bit-valid instance of its type.

#[doc(inline)]
pub use crate::transmute::{
    unsafe_transmute, UnsafeTransmuteFrom, UnsafeTransmuteInto, UnsafeTransmuteOptions,
};

/// Configuration options for ***sound*** transmutations.
pub mod neglect {
    #[doc(inline)]
    pub use crate::transmute::neglect::{Alignment, Stability, Transparency};
}
