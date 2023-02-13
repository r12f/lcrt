use std::{cell::RefCell, rc::Rc};

/// Definition for a binary tree node. The original definition is shown as below:
///
/// ```
/// #[derive(Debug, PartialEq, Eq)]
/// pub struct TreeNode {
///   pub val: i32,
///   pub left: Option<Rc<RefCell<TreeNode>>>,
///   pub right: Option<Rc<RefCell<TreeNode>>>,
/// }
///
/// impl TreeNode {
///   pub fn new(val: i32) -> Self { /* ... */ }
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[cfg(feature = "testing")]
    fn with_rc(val: i32) -> Rc<RefCell<TreeNode>> {
        Self::with_children(val, None, None)
    }

    #[cfg(feature = "testing")]
    fn with_children(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }

    /// Factory method to generate a tree from string. If string is empty, we return None.
    /// We could not use Into or TryInto traits here, because it doesn't return Option as we wanted here.
    #[cfg(feature = "testing")]
    pub fn from_str(raw_value: &str) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;

        let values: Vec<Option<i32>> = raw_value
            .split(',')
            .filter(|&x| !x.trim().is_empty())
            .map(|v| match v.trim() {
                "null" => None,
                v => Some(i32::from_str_radix(v, 10).unwrap()),
            })
            .collect();

        if values.is_empty() {
            return None;
        }

        let head = Self::with_rc(values[0].unwrap());
        let mut queue = VecDeque::new();
        queue.push_back(head.clone());

        let mut i = 1usize;
        while !queue.is_empty() {
            let parent_node = queue.pop_front().unwrap();
            if i < values.len() {
                if let Some(v) = values[i] {
                    let child = Self::with_rc(v);
                    queue.push_back(child.clone());
                    parent_node.borrow_mut().left = Some(child);
                }
                i += 1;
            } else {
                break;
            }

            if i < values.len() {
                if let Some(v) = values[i] {
                    let child = Self::with_rc(v);
                    queue.push_back(child.clone());
                    parent_node.borrow_mut().right = Some(child);
                }
                i += 1;
            } else {
                break;
            }
        }

        Some(head)
    }

    #[cfg(feature = "testing")]
    pub fn to_string(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        use std::collections::VecDeque;

        let mut result = String::new();

        let mut queue = VecDeque::new();
        let mut non_null_node_in_queue_count = if root.is_some() { 1 } else { 0 };
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            match node {
                Some(node) => {
                    non_null_node_in_queue_count -= 1;
                    result.push_str(&format!("{},", node.borrow().val));

                    queue.push_back(node.borrow().left.clone());
                    non_null_node_in_queue_count +=
                        if node.borrow().left.is_some() { 1 } else { 0 };

                    queue.push_back(node.borrow().right.clone());
                    non_null_node_in_queue_count +=
                        if node.borrow().right.is_some() { 1 } else { 0 };
                }
                None => {
                    result.push_str("null,");
                }
            }

            if non_null_node_in_queue_count == 0 {
                break;
            }
        }

        result = result.trim_end_matches(',').to_string();
        result
    }

    /// Asserts 2 trees are equal.
    #[cfg(feature = "testing")]
    pub fn assert_eq(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) {
        use pretty_assertions::assert_eq;

        let t1_str = Self::to_string(t1);
        let t2_str = Self::to_string(t2);
        assert_eq!(t1_str, t2_str);
    }

    /// Asserts 2 trees are equal.
    #[cfg(feature = "testing")]
    pub fn assert_list_eq(
        t1: Vec<Option<Rc<RefCell<TreeNode>>>>,
        t2: Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        use pretty_assertions::assert_eq;
        use std::collections::BTreeSet;

        let t1_set = t1
            .iter()
            .map(|v| Self::to_string(v.clone()))
            .collect::<BTreeSet<String>>();

        let t2_set = t2
            .iter()
            .map(|v| Self::to_string(v.clone()))
            .collect::<BTreeSet<String>>();

        assert_eq!(t1_set, t2_set);
    }
}

/// Macro for generate tree with vec-like syntax. If no parameter is provided, it returns None.
/// More on how Leetcode serializes the tree can be found here: https://support.leetcode.com/hc/en-us/articles/360011883654-What-does-1-null-2-3-mean-in-binary-tree-representation-
///
/// Example:
/// ```
/// lc_tree!(""); // This returns None.
/// lc_tree!("1,2,3"); // This returns a list: 1->(2, 3)
/// lc_tree!("1,null,2,3"); // This returns a list: 1->(null, 2->(3, null))
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_tree {
    ($te:literal) => {
        TreeNode::from_str($te)
    };
}

/// Macro for testing 2 trees are equal. The second parameter is provided in a way of string. If empty string is provided, it check against None.
///
/// Example:
/// ```
/// lc_tree_assert_eq(t, ""); // Check against None.
/// lc_tree_assert_eq(t, "1,2,3"); // Check against list: 1->(2, 3).
/// lc_tree_assert_eq(t, "1,null,2,3"); // Check against list: 1->(null, 2->(3, null)).
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_tree_assert_eq {
    ($t:expr, $te:literal) => {
        TreeNode::assert_eq($t, TreeNode::from_str($te))
    };
}

/// Macro for testing 2 list of trees are equal. The second parameter is provided in a way of list. If () is provided, it check against None.
///
/// Example:
/// ```
/// lc_tree_assert_list_eq(t, ()); // Check against None.
/// lc_tree_assert_list_eq(t, ("1,2,3")); // Check against list: 1->(2, 3).
/// lc_tree_assert_list_eq(t, ("1,2", "1,null,2,3")); // Check against list: 1->(2), 1->(null, 2->(3, null)).
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_tree_assert_list_eq {
    ($t:expr, ($($te:literal),*)) => {
        TreeNode::assert_list_eq($t, vec![$(TreeNode::from_str($te)),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use cool_asserts::assert_panics;
    use pretty_assertions::assert_eq;

    #[test]
    fn lc_tree_macro_can_generate_tree_from_empty_str() {
        let t = lc_tree!("");
        assert!(t.is_none());
    }

    #[test]
    fn lc_tree_macro_can_generate_tree_from_non_empty_str() {
        let t: Option<Rc<RefCell<TreeNode>>> = lc_tree!("1,null,2,3");
        assert!(t.is_some());

        let t = t.as_ref().unwrap().borrow();
        assert_eq!(t.val, 1);
        assert!(t.left.is_none());
        assert!(t.right.is_some());
        assert_eq!(t.right.as_ref().unwrap().borrow().val, 2);

        let trc = t.right.as_ref().unwrap().borrow();
        assert!(trc.left.is_some());
        assert_eq!(trc.left.as_ref().unwrap().borrow().val, 3);
        assert!(trc.right.is_none());

        let trc2 = trc.left.as_ref().unwrap().borrow();
        assert_eq!(trc2.val, 3);
        assert!(trc2.left.is_none());
        assert!(trc2.right.is_none());
    }

    #[test]
    fn tree_node_assert_eq_can_pass_on_identical_trees() {
        lc_tree_assert_eq!(lc_tree!(""), "");
        lc_tree_assert_eq!(lc_tree!("1"), "1");
        lc_tree_assert_eq!(lc_tree!("1,2,3"), "1,2,3");
        lc_tree_assert_eq!(lc_tree!("1,null,2,3"), "1,null,2,3");
        lc_tree_assert_eq!(lc_tree!("1,2,null,3"), "1,2,null,3");
    }

    #[test]
    fn tree_node_assert_eq_can_fail_on_non_identical_trees() {
        assert_panics!(lc_tree_assert_eq!(lc_tree!(""), "1"));
        assert_panics!(lc_tree_assert_eq!(lc_tree!("1"), ""));
        assert_panics!(lc_tree_assert_eq!(lc_tree!("1,null,2"), "1,2,null"));
        assert_panics!(lc_tree_assert_eq!(lc_tree!("1,null,2,3"), "1,2,null,3"));
    }

    #[test]
    fn tree_node_assert_list_eq_can_pass_on_identical_trees() {
        lc_tree_assert_list_eq!(vec![lc_tree!("")], (""));
        lc_tree_assert_list_eq!(vec![lc_tree!("1")], ("1"));
        lc_tree_assert_list_eq!(vec![lc_tree!("1"), lc_tree!("1,null,2")], ("1", "1,null,2"));
    }
}
