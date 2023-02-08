use crate::common::*;
use pretty_assertions::assert_eq;
use std::{env, fs};

#[allow(dead_code)]
pub fn assert_text_equal_with_baseline(actual_text: &str, reference_file_path: &str) {
    if env::var("GENERATE_TEST_DATA").is_ok() {
        fs::write(reference_file_path, actual_text).unwrap_or_else(|e| {
            panic!("Generate test data file failed! Path = {reference_file_path}, Error = {e}")
        });

        return;
    }

    let expected_text =
        fs::read_to_string(reference_file_path).expect("Reading reference file failed!");
    let expected_text = expected_text.replace("\r\n", "\n");

    assert_eq!(actual_text, &expected_text);
}

pub fn run_proc_macro_generation_test<T: TokenGenerator>(input: T, reference_file_path: &str) {
    let actual_tokens = input.to_tokens().to_string();
    let parsed_actual_tokens_for_format = syn::parse_file(&actual_tokens).unwrap();
    let actual_token_string = prettyplease::unparse(&parsed_actual_tokens_for_format);
    assert_text_equal_with_baseline(&actual_token_string, reference_file_path);
}
