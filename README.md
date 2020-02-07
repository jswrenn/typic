# Typic
Typic helps you transmute fearlessly. It worries about the subtleties of
***[soundness]*** and ***[safety]*** so you don't have to!

[![Documentation](https://docs.rs/typic/badge.svg)](https://docs.rs/typic/)
[![Crates.io](https://img.shields.io/crates/v/typic.svg)](https://crates.io/crates/typic/0.1.0)

Just import it and replace your `#[repr(...)]` attributes with `#[typic::repr(...)]`:
```rust
// Import it!
use typic::{self, TransmuteInto};

// Update your attributes!
#[typic::repr(C)]
pub struct Foo(pub u8, pub u16);

// Transmute fearlessly!
let _ : Foo = u32::default().transmute_into(); // Alchemy achieved!
let _ : u32 = Foo::default().transmute_into(); // Compiler Error!
```

[soundness]: https://docs.rs/typic/latest/typic/sound/
[safety]: https://docs.rs/typic/latest/typic/safe/

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>