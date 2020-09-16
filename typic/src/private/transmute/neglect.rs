/// Neglect statically guaranteeing pointer alignments.
///
/// By default, Typic ***statically requires*** that, when transmuting
/// between references, the destination type does not have stronger
/// alignment requirements than the source type.
///
/// This option is useful for implementing common fallible
/// transmutations, [like bytemuck's `try_cast_ref`](crate::extras::bytemuck::try_cast_ref).
///
/// This option is ***only*** available for ***unsafe*** transmutations,
/// since dereferencing an unaligned pointer may invoke UB.
pub struct Alignment;

/// Neglect statically guaranteeing that corresponding bytes in the source and destination
/// types have appropriate visibility.
///
/// Typic assumes that if a field of a type isn't `pub`, the type might
/// enforce invariants on its value. By default, Typic ***statically
/// requires*** that all bytes in the destination type are marked `pub`.
///
/// If you have special knowledge about the type (e.g., because you're
/// the author), you can opt-out of this guarantee by asserting
/// `T: UnsafeTransmuteInto<U, neglect::Transparency>`.
///
/// This option is ***only*** available for ***unsafe*** transmutations,
/// since calling methods on the transmuted reference may only be safe if
/// the type's internal invariants are upheld.
pub struct Transparency;

/// Neglect statically guaranteeing that the source and destination types have stable
/// in-memory representations.
///
/// By default, Typic ***statically requires*** that the layouts of the
/// source and destination types are part of their API guarantee. Library
/// authors indicate this for their types by implementing a marker trait.
/// This trait should only be implementable for types where Rust makes
/// guarantees about their layout (e.g., `#[repr(C)]`).
///
/// This is the only option available for safe transmutations.
///
/// Typic will ***still*** reject transmutes between types where the
/// layouts of the source or destination types are compiler UB (e.g.,
/// most `repr(Rust)` types).
pub struct Stability;

/// Neglect statically guaranteeing that all instances of the source type are bit-valid
/// instances of the destination type.
///
/// (Typic does not currently fully implement this, but will soon.)
///
/// By default, Typic only accepts transmutations for which all possible
/// values of the source type are bit-valid values of the destination
/// type.  (This means no `u8 → bool` transmutes!)
///
/// If you have special knowledge about the value (e.g., because you've
/// ensured at runtime that it's a bit-valid instance of the destination
/// type), you can opt-out of this guarantee by asserting
/// `T: UnsafeTransmuteInto<U, neglect::Visibility>`.
///
/// Typic will still reject transmutations that cannot possibly be valid
/// for any value, e.g.:
/// ```ignore
/// #[typic::repr(C)] enum Foo { N = 24 }
/// #[typic::repr(C)] enum Bar { N = 24 }
///
/// let bar : Bar = (Foo::N).unsafe_transmute_into() // Compile error!
/// ```
pub struct Validity;

/// Options for safe and unsafe transmutation.
pub trait TransmuteOptions: UnsafeTransmuteOptions {
    type Stability;
}

impl TransmuteOptions for () {
    type Stability = super::Stable;
}

impl TransmuteOptions for Stability {
    type Stability = super::Unstable;
}

impl<O> UnsafeTransmuteOptions for O
where
    O: TransmuteOptions,
{
    type Alignment = super::Static;
    type Transparency = super::Enforced;
    type Stability = <O as TransmuteOptions>::Stability;
    type Validity = super::AlwaysValid;
}

/// Options for unsafe transmutation.
pub trait UnsafeTransmuteOptions {
    type Alignment;
    type Transparency;
    type Stability;
    type Validity;
}

impl UnsafeTransmuteOptions for Alignment {
    type Alignment = super::Unchecked;
    type Stability = super::Stable;
    type Transparency = super::Enforced;
    type Validity = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for (Alignment,) {
    type Alignment = super::Unchecked;
    type Stability = super::Stable;
    type Transparency = super::Enforced;
    type Validity = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for Transparency {
    type Alignment = super::Static;
    type Stability = super::Stable;
    type Transparency = super::Unenforced;
    type Validity = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for (Transparency,) {
    type Alignment = super::Static;
    type Stability = super::Stable;
    type Transparency = super::Unenforced;
    type Validity = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for Validity {
    type Alignment = super::Static;
    type Stability = super::Stable;
    type Transparency = super::Unenforced;
    type Validity = super::AlwaysValid;
}

impl UnsafeTransmuteOptions for (Validity,) {
    type Alignment = super::Static;
    type Stability = super::Stable;
    type Transparency = super::Enforced;
    type Validity = super::MaybeInvalid;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Stability) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Transparency, Stability) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Transparency, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Stability, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency, Stability) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Stability, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Transparency, Stability, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}

impl UnsafeTransmuteOptions for (Alignment, Transparency, Stability, Validity) {
    type Alignment = <Alignment as UnsafeTransmuteOptions>::Alignment;
    type Stability = <Stability as UnsafeTransmuteOptions>::Stability;
    type Transparency = <Transparency as UnsafeTransmuteOptions>::Transparency;
    type Validity = <Validity as UnsafeTransmuteOptions>::Validity;
}
