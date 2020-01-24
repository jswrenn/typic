pub mod bytes;
pub mod reference;

pub use bytes::Bytes;
pub use reference::{SharedRef, UniqueRef};

pub type PaddingSlot<S> = Bytes<bytes::kind::Uninitialized, S>;
pub type InitializedSlot<S> = Bytes<bytes::kind::Initialized, S>;
pub type NonZeroSlot<S> = Bytes<bytes::kind::NonZero, S>;
