#![allow(dead_code)]
#![allow(unused_variables)]

use proc_macro::TokenStream;

extern crate proc_macro;

mod common;
mod solution;

#[cfg(test)]
mod test_utils;

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    common::process_attribute_macro::<solution::SolutionSyn>(attr, item)
}
