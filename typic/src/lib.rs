#[doc(hidden)]
#[deprecated(note = "TODO")]
pub enum TODO {}

pub(crate) mod bytelevel;
pub(crate) mod layout;
pub(crate) mod num;
pub(crate) mod target;
pub(crate) mod transmute;

pub mod highlevel;

#[doc(inline)]
pub use transmute::Transmute;

#[doc(inline)]
pub use typic_derive::repr;
