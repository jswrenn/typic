# Typic

Type-safe transmutations between layout-compatible types. See [./typic/tests/] for usage examples. **Note: This is a minimally viable proof-of-concept and it not suitable for real-world use.**

## How?

To provide type-safe transmutations, Typic must reason about the memory layout of types at type-checking time. Unfortunately, `rustc` computes the layout of types _after_ type-checking. To make this information available at type-checking time, Typic encodes `rustc`'s `repr(C)` layout algorithm as a trait resolution problem.

For types marked `#[typic::repr(C)]`, typic computes a type-level representations of their memory layout. This representation encodes whether byte sequences of that type's layout are arbitrary initialized bytes (equivalent to `u8`), non-zero initialized bytes (equivalent to `NonZeroU8`), padding bytes (equivalent to `MaybeUninit<u8>`), or references to other types.

To transmute between two `#[typic::repr(C)]` types, Typic compares these type-level layout representations in lock-step to test if they are layout-compatible. For instance, initialized bytes in the `Source` type may be mapped to uninitialized bytes in the `Destination` type, but not visa versa—that would unsoundly expose uninitialized bytes as if they were initialized.

## Limitations

Typic does not (and will probably never) support self-referential types, e.g.:

```rust
struct Foo<'a>(&'a Foo)
```

Typic does not currently representations other than plain `repr(C)` (e.g., alternative alignments or the `packed` modifier), `enum`s, or `union`s. These layout algorithms have merely not been implemented yet.
