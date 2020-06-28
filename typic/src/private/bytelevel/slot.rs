pub mod array;
pub mod bytes;
pub mod reference;

pub use array::Array;
pub use bytes::Bytes;
pub use reference::{Reference, Shared, SharedRef, Unique, UniqueRef};

/// The data is from a `pub` field
pub type Pub = crate::internal::Public;

/// The field is from a field that is not `pub`. 
pub type Priv = crate::internal::Private;

pub type PaddingSlot<Vis, S> = Bytes<Vis, bytes::kind::Uninitialized, S>;
pub type InitializedSlot<Vis, S> = Bytes<Vis, bytes::kind::Initialized, S>;
pub type NonZeroSlot<Vis, S> = Bytes<Vis, bytes::kind::NonZero, S>;
