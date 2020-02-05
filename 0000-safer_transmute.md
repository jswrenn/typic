- Feature Name: `safer_transmute`
- Start Date: (fill me in with today's date, YYYY-MM-DD)
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

# Summary
[summary]: #summary

A public API for statically-provable safe and sound transmutation between types.

***The examples in this RFC are fully implemented by [Typic](https://github.com/jswrenn/typic).*** For examples with type definitions, merely replace `#[repr(...)]` with `#[typic::repr(...)]`.

# Motivation
[motivation]: #motivation

Why are we doing this? What use cases does it support? What is the expected outcome?

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

Transmutation is an operation that re-interprets bytes belonging to a value of one type (henceforth `T`) as if they were bytes belonging to a value of a different type (henceforth `U`). Rust provides three mechanisms for transmuting values:

## Types of Transmutation

### Unsound Transmutation (`transmute`, `transmute_copy`)
The \[existing\] `mem::transmute` and `mem::transmute_copy` intrinsics allow for the ***unsafe*** and ***unsound*** transmutation between any `T` and `U`.

These intrinsics are deeply unsafe. The Rust compiler will accept uses of these intrinsics even when `T` and `U` do not have well-defined layouts. ***Always use a [safe transmutation](#safe-transmutation) method instead, if possible.*** If you are unable to use a safe transmutation method, ***you may be relying on undefined compiler behavior***.

### Sound Transmutation (`sound_transmute`)
The `mem::sound_transmute` function allows for the ***unsafe*** transmutation between `T` and `U`, when merely transmuting from `T` to `U` will not cause undefined behavior. For the key rules that govern when `T` is soundly convertible to `U`, see ***[When is a transmutation sound?][understanding-soundness]***.

This operation is `unsafe`, as it will bypass any user-defined validity restrictions that `U` places on its fields and enforces with its constructors and methods.  ***Always use a [safe transmutation](#safe-transmutation) method instead, if possible.*** If you are unable to use a safe transmutation method, you may be violating library invariants.

### Safe Transmutation (`TransmuteInto` and `TransmuteFrom`)
The `TransmuteInto<U>` trait is implemented for a types `T` and `U` if `T` is ***safely*** and ***soundly*** transmutable into `U`. This trait is only implemented when:
1. [`T` is soundly transmutable into `U`][understanding-soundness]
2. `U` does not have any inter-field validity constraints (i.e., it's `Transparent`)

The `Transparent` marker trait indicates that a type does not have any inter-field validity requirements. This trait is implemented for all primitive types.

The `Transparent` trait is implemented automatically for all `struct` and `enum` types whose fields have the same visibility as the type definition itself. These types are `Transparent` because anywhere the type is in scope, so to are its fields via the `.` operator.

In cases where you do not wish to increase the visibility of your type's fields, you may manually implement this trait for your type.


## When is a transmutation sound?
[understanding-soundness]: #when-is-a-transmutation-sound

### Well-Specified Representation
Transmutation is ***always*** unsound if it occurs between types with unspecified representations. Most of Rust's primitive types have specified representations. That is, the layout characteristics of `u8`, `f32` and others are guaranteed to be stable across compiler versions.

In contrast, most `struct` and `enum` types defined without an explicit `#[repr(C)]` or `#[repr(transparent)]` attribute do ***not*** have well-specified layout characteristics.

To ensure that types you've define are soundly transmutable, you usually must mark them with the `#[repr(C)]` attribute.

### Transmuting Owned Values
Transmutations into owned types must adhere to three conditions:

#### Broadening (or Preserving) Bit Validity
For each _i<sup>th</sup>_ of the destination type, all possible instantiations of the _i<sup>th</sup>_ byte of the source type must be a bit-valid instance of the _i<sup>th</sup>_ byte of the destination type.

For example, we are permitted us to transmute a `NonZeroU8` into a `u8`:
```rust
let _ : u8 = NonZeroU8::new(1).unwrap().transmute_into();
```
...because all possible instances of `NonZeroU8` are also valid instances of `u8`. However, transmuting a `u8` into a `NonZeroU8` is forbidden:
```rust
let _ : NonZeroU8 = u8::default().transmute_into(); // Compile Error!
```
...because not all instances of `u8` are valid instances of `NonZeroU8`.

Another example: While laying out certain types, rust may insert padding bytes between the layouts of fields. In the below example `Padded` has two padding bytes, while `Packed` has none:
```rust
#[repr(C)]
#[derive(Default)]
struct Padded(pub u8, pub u16, pub u8);

#[repr(C)]
#[derive(Default)]
struct Packed(pub u16, pub u16, pub u16);

assert_eq!(mem::size_of::<Packed>(), mem::size_of::<Padded>());
```

We may safely transmute from `Packed` to `Padded`:
```rust
let _ : Padded = Packed::default().transmute_into();
```
...but not from `Padded` to `Packed`:
```rust
let _ : Packed = Padded::default().transmute_into(); // Compile Error!
```
...because doing so would expose two uninitialized padding bytes in `Padded` as if they were initialized bytes in `Packed`. 

#### Shrinking (or Preserving) Size
It's completely safe to transmute into a type with fewer bytes than the destination type; e.g.:
```rust
let _ : u8 = u64::default().transmute_into();
```
This transmute truncates away the final three bytes of the `u64` value.

A value may ***not*** be transmuted into a type of greater size:
```rust
let _ : u8 = u64::default().transmute_into(); // Compile Error!
```


### Transmuting References
The restrictions above that apply to transmuting owned values, also apply to transmuting references. However, references carry a few additional restrictions:

#### Relaxing (or Preserving) Alignment
You may transmute a reference into reference of more relaxed alignment:
```rust
let _: &[u8; 0] = (&[0u16; 0]).transmute_into();
```

However, you may **not** transmute a reference into a reference of stricter alignment:
```rust
let _: &[u16; 0] = (&[0u8; 0]).transmute_into(); // Compile Error!
```

#### Shrinking (or Preserving) Lifetimes
You may transmute a reference into reference of lesser lifetime:
```rust
let _: &[u8; 0] = (&'static [0u16; 0]).transmute_into();
```

However, you may **not** transmute a reference into a reference of greater lifetime:
```rust
let _: &'static [u16; 0] = (&[0u8; 0]).transmute_into(); // Compile Error!
```

#### Shrinking (or Preserving) Mutability
You may preserve or decrease the mutability of a reference through transmutation:
```rust
let _: &u8 = (&u8).transmute_into();
let _: &u8 = (&mut u8).transmute_into();
```

However, you may **not** transmute an immutable reference into a mutable reference:
```rust
let _: &mut u8 = (&u8).transmute_into(); // Compile Error!
```

#### Preserving Validity
Unlike transmutations of owned values, the transmutation of a reference may also not expand the bit-validity of the referenced type. For instance:

```rust
let mut x = NonZeroU8::new(42).unwrap();

{
    let y : &mut u8 = (&mut x).transmute_into(); // Compile Error!
    *y = 0;
}

let z : NonZeroU8 = x; 
```
If this example did not produce a compile error, the value of `z` would not be a bit-valid instance of its type.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

## Library Additions

### `unsafe fn sound_transmute<T, U>(T) -> U`

```rust
pub unsafe fn sound_transmute<T, U>(from: T) -> U
where
    U: FromType<T>,
{
    let to = mem::transmute_copy(&from);
    mem::forget(from);
    to
}
```

### `unsafe trait Transparent`
```rust
#[marker]
pub unsafe trait Transparent {}
```
A type may implement the marker trait `Transparent` if it does not maintain any inter-field validity requirements.

This trait is implemented automatically for `enum` and `struct` items whose fields all share the same visibility as the item itself. Such items cannot uphold interfield validity requirements via their constructors and methods, because any code where the item is visible can safely mutate its equally-visible fields via dot-access.

### `trait TransmuteFrom<T>`
`TransmuteFrom` allows for the **safe** transmutation of a value between two types. 
```rust
pub trait TransmuteFrom<T>: Sized {
    /// Performs the conversion.
    fn transmute_from(from: T) -> Self;
}
```

A type `T` is safely transmutable to a type `U` if both:
1. `U` does not maintain any inter-field validity requirements (`U: Transparent`)
2. Every instance of `T` is a bit-valid instance of `U` (`U: FromType<T>`).

Accordingly, we provide this blanket implementation:

```rust
impl<T, U> TransmuteFrom<T> for U
where
    U: Transparent + FromType<T>,
{
    #[inline(always)]
    fn transmute_from(from: T) -> U {
        unsafe { sound_transmute(from) }
    }
}
```


### `trait TransmuteInto<U>`
For consistency with [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`], we provide the trait `TransmuteInto`, the reciprocal of `TransmuteFrom`:

```rust
pub trait TransmuteInto<U>: Sized {
    /// Performs the conversion.
    fn transmute_into(self) -> U;
}
```

[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html

[`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html

Accordingly, we provide this blanket implementation:

```rust
impl<T, U> TransmuteInto<U> for T
where
    U: TransmuteFrom<T>,
{
    #[inline(always)]
    fn transmute_into(self) -> U {
        U::transmute_from(self)
    }
}
```

### `unsafe trait FromType<T>` (Perma-Unstable)
```rust
#[marker]
#[unstable]
pub trait FromType<T> {}
```

A type `U` is `FromType<T>` if every possible instance of `T` is specified to be **soundly** transmutable into `U`. This trait shall be permenantly unstable. The compiler shall resolve whether `U: FromType<T>` holds for two concrete types `U` and `T` during trait resolution.

`FromType<T>` shall be implemented incrementally and conservatively. 

# Drawbacks
[drawbacks]: #drawbacks

Increased compiler complexity.

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

[`FromBits`]: https://internals.rust-lang.org/t/pre-rfc-frombits-intobits/7071
[Safe Transmute v2]: https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
[`Compatible`]: https://gist.github.com/gnzlbg/4ee5a49cc3053d8d20fddb04bc546000

## Familiar
The public API defined by this proposal is consistent with Rust's existing core conversion traits, [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`].

## Conservative
This proposal is conservative, only providing a public API transmutations whose soundness is _statically_ provable. Falliable conversions, such as `u8` to `bool` are not supported.

In contrast, the [`Compatible`] proposal recommends the simultaneous addition of a `TryCompatible` trait for fallible conversions. 

## Expressive and Incremental
The public API defined by this proposal allows for the eventual support of safer transmutation in all cases that can be statically provably sound. The transmutations supported by this proposal may be incrementally increased without altering the public API by adding additional implementations of `FromType`.

In contrast, in the [`FromBits`] and [Safe Transmute v2] proposal's:
* `FromBits`/`FromAnyBytes` only supports conversions into types with validity no stricter than arbitrarily-initialized.
* `IntoBits`/`ToBytes` only supports types without padding or other sources of uninitialized bytes.

The expressive power of these traits cannot be substantially increased without providing a different public API for conversion. 

# Prior art
[prior-art]: #prior-art

TODO

# Unresolved questions
[unresolved-questions]: #unresolved-questions

TODO

# Future possibilities
[future-possibilities]: #future-possibilities

TODO