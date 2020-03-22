/// Neglect statically guaranteeing pointer alignments.
pub struct Alignment;

/// Neglect guaranteeing that corresponding bytes in the source and destination
/// types have appropriate visibility.
pub struct Transparency;

/// Neglect guaranteeing that the source and destination types have stable
/// in-memory representations.
pub struct Stability;

/// Neglect guaranteeing that all instances of the source type are bit-valid
/// instances of the destination type. 
pub struct Validity;

/// Options for safe and unsafe transmutation.
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
    type Validity       = super::AlwaysValid;
}

/// Options for unsafe transmutation.
pub trait UnsafeTransmuteOptions {
    type Alignment;
    type Transparency;
    type Stability;
    type Validity;
}

impl UnsafeTransmuteOptions for Alignment {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
    type Validity     = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for (Alignment,) {
    type Alignment    = super::Unchecked;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
    type Validity     = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for Transparency {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
    type Validity     = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for (Transparency,) {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
    type Validity     = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for Validity {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Unenforced;
    type Validity     = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for (Validity,) {
    type Alignment    = super::Static;
    type Stability    = super::Stable;
    type Transparency = super::Enforced;
    type Validity     = super::MaybeInvalid;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Stability) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Transparency, Stability) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Transparency, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Stability, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency, Stability) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Stability, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Transparency, Stability, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}


impl UnsafeTransmuteOptions for (Alignment, Transparency, Stability, Validity) {
    type Alignment    = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability    = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity     = <Validity as UnsafeTransmuteOptions>::Validity;
}