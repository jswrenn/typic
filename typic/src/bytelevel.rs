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
pub type NonZeroSeq<S, Rest> =
    PCons<slot::NonZeroSlot<U1>, PCons<slot::InitializedSlot<Sub1<S>>, Rest>>;

#[cfg(target_endian = "big")]
pub type NonZeroSeq<S, Rest> =
    PCons<slot::InitializedSlot<Sub1<S>, PCons<slot::NonZeroSlot<U1>>, Rest>>;

pub type ReferenceBytes<Rest> = NonZeroSeq<PointerWidth, Rest>;
