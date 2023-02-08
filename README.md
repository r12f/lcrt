# LCRT (Leetcode Runtime)

[![Crates.io](https://img.shields.io/crates/v/lcrt)](https://crates.io/crates/lcrt)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)

LCRT is a really simple utility crate to help writing leetcode solutions in rust. It leverages proc macros in rust to reduce the boilerplate code that we need to write.

## How to use

To use this library, please add the following into the dependencies in the `Cargo.toml` file:

```toml
[dependencies]
lcrt-macro = "0.1"
lcrt = "0.1"
```

Then in your local library, in `lib.rs` file, please add the macro use as below.

```rust
#[macro_use]
extern crate lcrt_macro;
```

Then we can start writing code like this:

```rust
#[solution]
impl Solution {
    pub fn add_two_numbers(
        _l1: Option<Box<ListNode>>,
        _l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        None
    }
}
```

Underlying, it will alternate the code as below:

```rust
mod p2_add_two_numbers {
    use lcrt::*;
    pub struct Solution {}
    impl Solution {
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
        }
    }
}
```

## License

Apache-2.0: <https://www.apache.org/licenses/LICENSE-2.0>