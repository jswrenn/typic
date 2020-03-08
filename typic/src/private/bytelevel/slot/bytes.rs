//! Non-reference slots.
use core::marker::PhantomData;

/// A sequence of bytes of `Kind` and `Size`.
pub struct Bytes<Vis, Kind, Size>(PhantomData<(Vis, Kind, Size)>);

/// Markers indicating the kind of bit-level validity restrictions that exist
/// on a `Bytes`.
pub mod kind {
    /// The byte(s) must be initialized to a non-zero value.
    pub struct NonZero;

    /// The byte(s) must be initialized to any value.
    pub struct Initialized;

    /// The byte(s) may be uninitialized or initialized.
    pub struct Uninitialized;
}
