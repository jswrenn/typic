/// Neglect statically guaranteeing pointer alignments.
pub struct Alignment;

/// Neglect guaranteeing that corresponding bytes in the source and destination
/// types have appropriate visibility.
pub struct Transparency;

/// Neglect guaranteeing that the source and destination types have stable
/// in-memory representations.
pub struct Stability;

pub trait Options: UnsafeOptions {
    type Stability;
}

impl Options for () {
    type Stability    = super::Stable;
}

impl Options for Stability {
    type Stability    = super::Unstable;
}

impl<O> UnsafeOptions for O
where
    O: Options
{
    type Alignment      = super::Static;
    type Transparency   = super::Enforced;
    type Stability      = <O as Options>::Stability;
}

pub trait UnsafeOptions {
    type Alignment;
    type Transparency;
    type Stability;
}

impl UnsafeOptions for Alignment {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
}

impl UnsafeOptions for (Alignment,) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
}

impl UnsafeOptions for Transparency {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeOptions for (Transparency,) {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeOptions for (Alignment, Transparency) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeOptions for (Transparency, Alignment) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
}

impl UnsafeOptions for (Alignment, Stability) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Unstable;
    type Transparency = super::Enforced;
}

impl UnsafeOptions for (Stability, Alignment) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Unstable;
    type Transparency = super::Enforced;
}

impl UnsafeOptions for (Stability, Transparency) {
    type Alignment    = super::Static;
    type Stability    = super::Unstable;
    type Transparency = super::Unenforced;
}

impl UnsafeOptions for (Alignment, Transparency, Stability) {
    type Alignment    = <Alignment as UnsafeOptions>::Alignment;
    type Stability    = <Stability as UnsafeOptions>::Stability;
    type Transparency = <Transparency as UnsafeOptions>::Transparency;
}

impl UnsafeOptions for (Stability, Alignment, Transparency) {
    type Alignment    = <Alignment as UnsafeOptions>::Alignment;
    type Stability    = <Stability as UnsafeOptions>::Stability;
    type Transparency = <Transparency as UnsafeOptions>::Transparency;
}

impl UnsafeOptions for (Transparency, Stability, Alignment) {
    type Alignment    = <Alignment as UnsafeOptions>::Alignment;
    type Stability    = <Stability as UnsafeOptions>::Stability;
    type Transparency = <Transparency as UnsafeOptions>::Transparency;
}

impl UnsafeOptions for (Transparency, Alignment, Stability) {
    type Alignment    = <Alignment as UnsafeOptions>::Alignment;
    type Stability    = <Stability as UnsafeOptions>::Stability;
    type Transparency = <Transparency as UnsafeOptions>::Transparency;
}

impl UnsafeOptions for (Alignment, Stability, Transparency) {
    type Alignment    = <Alignment as UnsafeOptions>::Alignment;
    type Stability    = <Stability as UnsafeOptions>::Stability;
    type Transparency = <Transparency as UnsafeOptions>::Transparency;
}

impl UnsafeOptions for (Stability, Transparency, Alignment) {
    type Alignment    = <Alignment as UnsafeOptions>::Alignment;
    type Stability    = <Stability as UnsafeOptions>::Stability;
    type Transparency = <Transparency as UnsafeOptions>::Transparency;
}
