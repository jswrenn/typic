//! Traits for safe and sound transmutation.
//!
//! [soundness]: crate::transmute::unsafe_transmutation#when-is-a-transmutation-sound
//! [safety]: crate::transmute::safe_transmutation
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
//! [`unsafe_transmute`]: self::unsafe_transmute
//!
//! The [`unsafe_transmute`] function allows for the ***unsafe*** transmutation
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
//! [`TransmuteInto<U>`]: self::TransmuteInto
//!
//! The [`TransmuteInto<U>`] trait is implemented for a type `T` if:
//! 1. [`T` is ***soundly*** transmutable into `U`][soundness], and
//! 2. [`T` is ***safely*** transmutable into `U`][safety].
//!
//! If you are unable to use [`TransmuteInto<U>`], you may be attempting a
//! transmutation that is relying unspecified behavior.

pub mod safe_transmutation;
pub mod unsafe_transmutation;

#[doc(inline)]
pub use crate::private::transmute::{
    safe_transmute,
    StableTransmuteInto,
    TransmuteFrom,
    TransmuteInto,
    neglect::TransmuteOptions,

    unsafe_transmute,
    UnsafeTransmuteFrom,
    UnsafeTransmuteInto,
    neglect::UnsafeTransmuteOptions,
};

/// What static checks should Typic neglect?
pub mod neglect {
    #[doc(inline)]
    pub use crate::private::transmute::neglect::{
        Alignment,
        Stability,
        Transparency,
    };
}
