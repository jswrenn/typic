//! Guidance and tools for ***safe*** transmutation.
//!
//! A [sound transmutation] is safe only if the resulting value cannot possibly
//! violate library-enforced invariants. Typic assumes that all non-zero-sized
//! fields with any visibility besides `pub` could have library-enforced
//! invariants.
//!
//! [sound transmutation]: crate#sound-transmutation
//! [soundness]: crate::sound#when-is-a-transmutation-sound
//! [`TransmuteInto`]: crate::TransmuteInto
//! [`unsafe_transmute`]: crate::unsafe_transmute
//!
//! ## Why is safety different than soundness?
//! Consider the type `Constrained`, which enforces a validity constraint on its
//! fields, and the type `Unconstrained` (which has no internal validity
//! constraints):
//!
//! ```
//! # use typic::docs::prelude::*;
//! #[typic::repr(C)]
//! #[derive(StableABI)]
//! pub struct Constrained {
//!     wizz: i8,
//!     bang: u8,
//! }
//!
//! impl Constrained {
//!     /// the sum of `wizz` and `bang` must be greater than or equal to zero.
//!     pub fn new(wizz: i8, bang: u8) -> Self {
//!         assert!((wizz as i16) / (bang as i16) >= 0);
//!         Constrained { wizz, bang }
//!     }
//!
//!     pub fn something_dangerous(&self) {
//!         unsafe {
//!             // do something that's only safe if `wizz + bang >= 0`
//!         }
//!     }
//! }
//!
//! #[typic::repr(C)]
//! #[derive(StableABI)]
//! pub struct Unconstrained {
//!     pub wizz: u8,
//!     pub bang: i8,
//! }
//! ```
//!
//! It is [sound][soundness] to transmute an instance of `Unconstrained` into
//! `Constrained`:
//! ```
//! use typic::docs::prelude::*;
//! use typic::transmute::neglect;
//! let _ : Constrained  = unsafe { unsafe_transmute::<_, _, neglect::Transparency>(Unconstrained::default()) };
//! ```
//! ...but it is **not** safe! The [`unsafe_transmute`] function creates an
//! instance of `Bar` _without_ calling its `new` constructor, thereby bypassing
//! the safety check which ensures `something_dangerous` does not violate Rust's
//! memory model. The compiler will reject our program if we try to safely
//! transmute `Unconstrained` to `Constrained`:
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let unconstrained = Unconstrained::default();
//! let _ : Constrained  = unconstrained.transmute_into();
//! ```
//!
//! Or, ***automatically***, by marking the fields `pub`:
//! ```
//! # use typic::docs::prelude::*;
//! #[typic::repr(C)]
//! #[derive(StableABI)]
//! pub struct Unconstrained {
//!     pub wizz: u8,
//!     pub bang: i8,
//! }
//!
//! let _ : Unconstrained = u16::default().transmute_into();
//! ```
//!
//! If the fields are marked `pub`, the type cannot possibly rely on any
//! internal validity requirements, as users of the type are free to manipulate
//! its fields direclty via the `.` operator.
//!
//! ## Safely transmuting references
//! When safely transmuting owned values, all non-padding bytes in the source
//! type must correspond to `pub` bytes in the destination type:
//! ```
//! # use typic::docs::prelude::*;
//! let _ : Unconstrained = Constrained::default().transmute_into();
//! ```
//! The visibility (or lack thereof) of bytes in the source type does not
//! affect safety.
//!
//! When safely transmuting references, each corresponding byte in the source
//! and destination types must have the _same_ visibility. Without this
//! restriction, you could inadvertently violate library invariants of a type
//! by transmuting and mutating a mutable reference to it:
//!
//! ```compile_fail
//! # use typic::docs::prelude::*;
//! let mut x = Constrained::default();
//!
//! {
//!     let y : &mut Unconstrained = (&mut x).transmute_into();
//!                                        // ^^^^^^^^^^^^^^
//!                                        // Compile Error!
//!     let z : u8 = -100i8.transmute_into();
//!     y.wizz = z;
//! }
//!
//! // Ack! `x.wizz + x.bang` is now -100!
//! // This violates the safety invariant of `something_dangerous`!
//! x.something_dangerous();
//! ```

pub use super::{
    safe_transmute,
    TransmuteFrom,
    TransmuteInto,
    StableTransmuteInto,
    TransmuteOptions
};

/// Configuration options for ***safe*** transmutations.
pub mod neglect {
    pub use crate::transmute::neglect::Stability;
}
