/// Neglect statically guaranteeing pointer alignments.
pub struct Alignment;

/// Neglect guaranteeing that corresponding bytes in the source and destination
/// types have appropriate visibility.
pub struct Transparency;

/// Neglect guaranteeing that the source and destination types have stable
/// in-memory representations.
pub struct Stability;

pub trait TransmuteOptions: UnsafeTransmuteOptions {
    type Stability;
}

impl TransmuteOptions for () {
    type Stability    = super::Stable;
}

impl TransmuteOptions for Stability {
    type Stability    = super::Unstable;
}

impl<O> UnsafeTransmuteOptions for O
where
    O: TransmuteOptions
{
    type Alignment      = super::Static;
    type Transparency   = super::Enforced;
    type Stability      = <O as TransmuteOptions>::Stability;
}

pub trait UnsafeTransmuteOptions {
    type Alignment;
    type Transparency;
    type Stability;
}

impl UnsafeTransmuteOptions for Alignment {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
}

impl UnsafeTransmuteOptions for (Alignment,) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
}

impl UnsafeTransmuteOptions for Transparency {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeTransmuteOptions for (Transparency,) {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeTransmuteOptions for (Transparency, Alignment) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeTransmuteOptions for (Alignment, Stability) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Unstable;
    type Transparency = super::Enforced;
}

impl UnsafeTransmuteOptions for (Stability, Alignment) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Unstable;
    type Transparency = super::Enforced;
}

impl UnsafeTransmuteOptions for (Stability, Transparency) {
    type Alignment    = super::Static;
    type Stability    = super::Unstable;
    type Transparency = super::Unenforced;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency, Stability) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
}

impl UnsafeTransmuteOptions for (Stability, Alignment, Transparency) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
}

impl UnsafeTransmuteOptions for (Transparency, Stability, Alignment) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
}

impl UnsafeTransmuteOptions for (Transparency, Alignment, Stability) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
}

impl UnsafeTransmuteOptions for (Alignment, Stability, Transparency) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
}

impl UnsafeTransmuteOptions for (Stability, Transparency, Alignment) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
}
