//! The byte-level representation of a type.

pub mod coproduct;
pub mod ops;
pub mod product;
pub mod slot;

pub use ops::{Add, Sum};
pub use product::{Cons as PCons, Nil as PNil};

use crate::num::{Sub1, U1};
use crate::target::PointerWidth;

#[cfg(target_endian = "little")]
#[allow(unused)]
pub type ReferenceBytes<Rest> =
  PCons<slot::NonZeroSlot<U1>,
    PCons<slot::InitializedSlot<Sub1<PointerWidth>>,
      Rest>>;

#[cfg(target_endian = "big")]
#[allow(unused)]
pub type ReferenceBytes<Rest> =
  PCons<slot::InitializedSlot<Sub1<PointerWidth>,
    PCons<slot::NonZeroSlot<U1>>,
      Rest>>;

