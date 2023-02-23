/// Macro for generate a vector of string with vec-like syntax.
///
/// Example:
/// ```
/// lc_strvec![["eat","tea","tan","ate","nat","bat"]]; // This creates a Vec<Vec<String>>.
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_strvec {
    ($($e:tt),*) => {
        vec![$($e.to_string()),*]
    };
}

/// Macro for generate a vector inside of a vector with vec-like syntax.
///
/// Example:
/// ```
/// lc_vecvec![[9], [3, 15], [20], [7]]; // This creates a Vec<Vec<i32>>.
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_vecvec {
    ($($e:tt),*) => {
        vec![$($e.to_vec()),*]
    };
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn lc_strvec_macro_can_generate_string_vec() {
        let l = lc_strvec!["eat", "tea", "tan", "ate", "nat", "bat"];
        assert_eq!(
            l,
            vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]
        );
    }

    #[test]
    fn lc_vecvec_macro_can_generate_vec_vec() {
        let l = lc_vecvec![[9], [3, 15], [20], [7]];
        assert_eq!(l, vec![vec![9], vec![3, 15], vec![20], vec![7]]);
    }
}
