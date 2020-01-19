/// A marker indicating that a compound type is `#[repr(packed)]`
pub struct Packed;

/// A marker indicating that a compound type is not `#[repr(packed)]`.
pub struct Padded;

/// A trait defining the set of possible padding modes.
pub trait Padding {}

impl Padding for Packed {}

impl Padding for Padded {}
