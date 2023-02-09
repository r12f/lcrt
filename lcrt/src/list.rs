/// Definition for singly-linked list. The original definition is shown as below:
///
/// ```
/// #[derive(PartialEq, Eq, Clone, Debug)]
/// pub struct ListNode {
///   pub val: i32,
///   pub next: Option<Box<ListNode>>
/// }
///
/// impl ListNode {
///     pub fn new(val: i32) -> Self { /* ... */ }
/// }
/// ```
///
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Factory method to generate the list from vector. If vector is empty, we return None.
    /// We could not use Into or TryInto traits here, because it doesn't return Option as we wanted here.
    #[cfg(feature = "testing")]
    pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }

        let mut head = ListNode::new(0);
        let mut node = &mut head;
        for value in values {
            node.next = Some(Box::new(ListNode::new(value)));
            node = node.next.as_mut().unwrap().as_mut();
        }

        let ListNode { next: result, .. } = head;
        result
    }

    /// Asserts 2 lists are equal.
    #[cfg(feature = "testing")]
    pub fn assert_eq(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) {
        use pretty_assertions::assert_eq;

        if l1.is_none() && l2.is_none() {
            return;
        }

        assert!(!l1.is_none());
        assert!(!l2.is_none());

        let mut l1_node = l1.as_ref().unwrap();
        let mut l2_node = l2.as_ref().unwrap();

        loop {
            assert_eq!(l1_node.val, l2_node.val);
            assert_eq!(l1_node.next.is_some(), l2_node.next.is_some());

            if l1_node.next.is_none() {
                break;
            }

            l1_node = l1_node.next.as_ref().unwrap();
            l2_node = l2_node.next.as_ref().unwrap();
        }
    }
}

/// Macro for generate list with vec-like syntax. If no parameter is provided, it returns None.
///
/// Example:
/// ```
/// lc_list!(); // This returns None.
/// lc_list!(1, 2, 3); // This returns a list: 1->2->3
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_list {
    ($($e:expr),*) => {
        ListNode::from_vec(vec![$($e), *])
    };
}

/// Macro for testing 2 lists are equal. The second parameter is provided in a way of list. If no parameter is provided, it check against None.
///
/// Example:
/// ```
/// lc_list_assert_eq(l, ()); // Check against None.
/// lc_list_assert_eq(l, (1, 2, 3)); // Check against list: 1->2->3.
/// ```
#[cfg(feature = "testing")]
#[macro_export]
macro_rules! lc_list_assert_eq {
    ($l:expr, ($($e:expr),*)) => {
        ListNode::assert_eq($l, ListNode::from_vec(vec![$($e), *]))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use cool_asserts::assert_panics;
    use pretty_assertions::assert_eq;

    #[test]
    fn lc_list_macro_can_generate_list_from_empty_vector() {
        let l = lc_list![];
        assert!(l.is_none());
    }

    #[test]
    fn lc_list_macro_can_generate_list_from_non_empty_vector() {
        let l = lc_list![1, 2, 3];

        // First element
        assert!(l.is_some());
        assert_eq!(l.as_ref().unwrap().val, 1);

        // Second element
        assert!(l.as_ref().unwrap().next.is_some());
        assert_eq!(l.as_ref().unwrap().next.as_ref().unwrap().val, 2);

        // Third element
        assert!(l.as_ref().unwrap().next.as_ref().unwrap().next.is_some());
        assert_eq!(
            l.as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val,
            3
        );

        // End
        assert!(l
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .is_none());
    }

    #[test]
    fn list_node_assert_eq_can_check_identical_lists() {
        ListNode::assert_eq(lc_list!(), lc_list!());
        ListNode::assert_eq(lc_list!(1), lc_list!(1));
        ListNode::assert_eq(lc_list!(1, 2, 3), lc_list!(1, 2, 3));
    }

    #[test]
    fn list_node_assert_eq_can_fail_on_non_identical_lists() {
        assert_panics!(ListNode::assert_eq(lc_list!(), lc_list!(1)));
        assert_panics!(ListNode::assert_eq(lc_list!(1), lc_list!()));
        assert_panics!(ListNode::assert_eq(lc_list!(1, 2), lc_list!(1, 3)));
        assert_panics!(ListNode::assert_eq(lc_list!(1, 2), lc_list!(1, 2, 3)));
    }
}
