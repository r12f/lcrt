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

[dev-dependencies]
lcrt = { version = "0.1", features = ["testing"] }
```

> NOTE: The "testing" feature enables macros and utility functions for testing support. Those functions are placed after this feature in order to not mess up with the problem solving process.

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

## Unit test support

LCRT provides some utility functions below for writing tests locally. 

- `lc_list!(1, 2, 3, ...)`: Creates a linked list as `Option<Box<LinkedList>>`. If no parameter is provided, it returns None.
- `lc_list_assert_eq(list, (1, 2, 3, ...))`: Asserts 2 lists are equal. If second parameter is `()`, it asserts against None.
- `lc_tree!("1,null,2,3")`: Creates a tree as `Option<Rc<RefCell<TreeNode>>>`. For example, this creates `1->(null, 2->(3, null))`.
- `lc_tree_assert_eq(t, "1,null,2,3")`: Asserts 2 trees are equal. If second parameter is `""`, it asserts against None.
- `lc_tree_assert_list_eq(t, ("1,2", "1,null,2,3"))`: Asserts 2 tree lists are equal, ignoring order. For example, this checks against tree list: `1->(2), 1->(null, 2->(3, null))`.
- `lc_vecvec![[9], [3, 15], [20], [7]]`: Creates a nested vector as `Vec<Vec<i32>>`.

Here is an example, to write test for lists:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        lc_list_assert_eq!(Solution::add_two_numbers(lc_list!(), lc_list!()), ());
        lc_list_assert_eq!(Solution::add_two_numbers(lc_list!(1), lc_list!()), (1));
        lc_list_assert_eq!(Solution::add_two_numbers(lc_list!(1), lc_list!(1)), (2));

        lc_list_assert_eq!(
            Solution::add_two_numbers(lc_list!(2, 4, 3), lc_list!(5, 6, 4)),
            (7, 0, 8)
        );

        lc_list_assert_eq!(
            Solution::add_two_numbers(lc_list!(9, 9, 9, 9, 9, 9, 9), lc_list!(9, 9, 9, 9)),
            (8, 9, 9, 9, 0, 0, 0, 1)
        );
    }
}
```

## License

Apache-2.0: <https://www.apache.org/licenses/LICENSE-2.0>