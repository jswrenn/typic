//! Type-level information about the compilation target.

use crate::private::num::*;

/// The pointer width, in bytes, of the target platform.
#[cfg(target_pointer_width = "8")]
pub type PointerWidth = U1;
#[cfg(target_pointer_width = "16")]
pub type PointerWidth = U2;
#[cfg(target_pointer_width = "32")]
pub type PointerWidth = U4;
#[cfg(target_pointer_width = "64")]
pub type PointerWidth = U8;
#[cfg(target_pointer_width = "128")]
pub type PointerWidth = U16;
