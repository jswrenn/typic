pub mod padding;

pub use padding::PaddingNeededFor;

pub trait Layout {
    type Align;
    type ByteLevel;
}
