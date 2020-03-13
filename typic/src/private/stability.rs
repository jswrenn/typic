//! A type grammar for communicating layout guarantees.
//!
//! ```rust
//! use typic::{self, stability::*};
//!
//! #[typic::repr(C)]
//! struct Foo(u8, u16, u32);
//!
//! // Increasing `Foo`'s size from 8 bytes is a breaking change.
//! impl Never<Increase, Alignment> for Foo {}
//!
//! // Decreasing `Foo`'s minimum alignment from 4 is a breaking change. 
//! impl Never<Decrease, Size>      for Foo {}
//!
//! // Increasing `Foo`'s bit-validity is a breaking change. 
//! impl Never<Increase, Validity>  for Foo {}
//! ```

/// The minimum alignment aspect of a type's layout.
pub enum Alignment {}

/// The size aspect of a type's layout.
pub enum Size {}

/// The validity aspect of a type's layout.
pub enum Validity {}

/// Aspects of a type's layout.
///
/// The layout of a type has three aspects:
/// 1. [Minimum Alignment][Align]
/// 2. [Size][Size]
/// 3. [Validity][Validity]
pub trait Aspect: private::Sealed {}

impl Aspect for Alignment     {}
impl Aspect for Size      {}
impl Aspect for Validity  {}

/// Increasing the given aspect is a breaking change.
pub enum Increase {}

/// Decreasing the given aspect is a breaking change.
pub enum Decrease {}

/// The directionality of the layout guarantee.
pub trait Direction: private::Sealed {}

impl Direction for Increase {}
impl Direction for Decrease {}

/// Changing the `Aspect` in the `Direction` for `Self` is a breaking change.
pub trait Never<Direction, Aspect>
where
    Direction:  self::Direction,
    Aspect:     self::Aspect,
{}

mod private {
    use super::*;

    pub trait Sealed {}
    
    impl Sealed for Alignment   {}
    impl Sealed for Size        {}
    impl Sealed for Validity    {}
    
    impl Sealed for Increase    {}
    impl Sealed for Decrease    {}
}
