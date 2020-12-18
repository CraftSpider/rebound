
# Rebound

[![crates.io](https://img.shields.io/crates/v/rebound.svg)](https://crates.io/crates/rebound)
[![Documentation](https://docs.rs/rebound/badge.svg)](https://docs.rs/rebound)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/rebound.svg)](./LICENSE-APACHE)

A powerful, lifetime-safe runtime reflection system for Rust. **Heavily WIP, requires nightly**

## Goals

Rebound aims to provide high-power runtime reflection. This includes two main features:

- Reflect as much as possible. This means rebound is willing to trade some performance, code size, of simplicity
  in the name of increased power.
- Follow Rust's lifetime and memory safety guarantees. As long as you are using the safe API,
  rebound should never cause segfaults or any other data race.
  
## Features

Rebound currently provides these features:

- A `#[rebound]` proc macro, which can be applied to any item rebound supports reflecting
- Support for these items
  - Structs
  - Enums
  - All three kinds of generics
  - Impl Blocks
- Support for these things is being worked on
  - Unions
  - Traits
  - Top-level Functions
  - Statics
  - Consts
- The `Value` type, an untyped smart pointer with lifetime safety.
- Reflection of all primitive types, including the `!` type with feature-gate
- Reflection of the `core` and `std` builtin libraries, with feature-gates

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
