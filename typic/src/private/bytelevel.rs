//! The byte-level representation of a type.

pub mod coproduct;
pub mod ops;
pub mod product;
pub mod slot;

pub use ops::{Add, Sum};
pub use product::{Cons as PCons, Nil as PNil};

use crate::private::num::{Sub1, U1};
use crate::private::target::PointerWidth;

#[cfg(target_endian = "little")]
pub type NonZeroSeq<Vis, S, Rest> =
    PCons<slot::NonZeroSlot<Vis, U1>, PCons<slot::InitializedSlot<Vis, Sub1<S>>, Rest>>;

#[cfg(target_endian = "big")]
pub type NonZeroSeq<Vis, S, Rest> =
    PCons<slot::InitializedSlot<Vis, Sub1<S>, PCons<slot::NonZeroSlot<Vis, U1>>, Rest>>;

pub type ReferenceBytes<Vis, Rest> = NonZeroSeq<Vis, PointerWidth, Rest>;
