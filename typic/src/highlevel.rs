//! The byte-level representation of a type.

pub mod coproduct;
pub mod padding;
pub mod product;

pub trait Type {
    /// The padding mode of the type.
    type Padding: padding::Padding;

    /// An abstract representation of the type's structure.
    type Representation;
}
