//! G025: Binary tree maximum depth.

/// Owned binary tree node used for tree exercises.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    /// Construct a leaf node with no children.
    pub fn leaf(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    /// Construct a node with explicit left and right child subtrees.
    pub fn with_children(
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    ) -> Self {
        Self { value, left, right }
    }
}

/// Return the maximum depth (number of nodes along the longest root-to-leaf path).
/// Empty tree (`None`) returns `0`; a single leaf node returns `1`.
/// Node values may be duplicated, negative, or extreme `i32` values and do not affect depth.
/// Preserve the borrowed tree. Use safe Rust, `O(n)` time, and `O(h)` auxiliary space
/// (where `h` is the tree height, up to `O(n)` for skewed trees).
/// Either recursive or iterative traversal is allowed; no calls to earlier exercises.
pub fn max_depth(root: Option<&TreeNode>) -> usize {
    let _ = root;
    todo!("Implement binary tree maximum depth")
}

#[cfg(test)]
mod tests {
    use super::{TreeNode, max_depth};

    fn boxed(node: TreeNode) -> Option<Box<TreeNode>> {
        Some(Box::new(node))
    }

    fn leaf(value: i32) -> Option<Box<TreeNode>> {
        boxed(TreeNode::leaf(value))
    }

    #[test]
    fn empty_tree_has_zero_depth() {
        assert_eq!(max_depth(None), 0);
    }

    #[test]
    fn single_leaf_has_depth_one() {
        let root = TreeNode::leaf(42);
        assert_eq!(max_depth(Some(&root)), 1);
    }

    #[test]
    fn root_with_single_left_child() {
        let root = TreeNode::with_children(1, leaf(2), None);
        assert_eq!(max_depth(Some(&root)), 2);
    }

    #[test]
    fn root_with_single_right_child() {
        let root = TreeNode::with_children(1, None, leaf(2));
        assert_eq!(max_depth(Some(&root)), 2);
    }

    #[test]
    fn balanced_complete_tree() {
        let left = TreeNode::with_children(2, leaf(4), leaf(5));
        let right = TreeNode::with_children(3, leaf(6), leaf(7));
        let root = TreeNode::with_children(1, boxed(left), boxed(right));
        assert_eq!(max_depth(Some(&root)), 3);
    }

    #[test]
    fn asymmetric_deeper_left_subtree() {
        let deep_left =
            TreeNode::with_children(2, boxed(TreeNode::with_children(4, leaf(7), None)), leaf(5));
        let root = TreeNode::with_children(1, boxed(deep_left), leaf(3));
        assert_eq!(max_depth(Some(&root)), 4);
    }

    #[test]
    fn asymmetric_deeper_right_subtree() {
        let deep_right = TreeNode::with_children(
            3,
            None,
            boxed(TreeNode::with_children(
                5,
                boxed(TreeNode::with_children(6, None, leaf(8))),
                None,
            )),
        );
        let root = TreeNode::with_children(1, leaf(2), boxed(deep_right));
        assert_eq!(max_depth(Some(&root)), 5);
    }

    #[test]
    fn left_skewed_chain() {
        let mut current = TreeNode::leaf(50);
        for value in (1..50).rev() {
            current = TreeNode::with_children(value, boxed(current), None);
        }
        assert_eq!(max_depth(Some(&current)), 50);
    }

    #[test]
    fn right_skewed_chain() {
        let mut current = TreeNode::leaf(50);
        for value in (1..50).rev() {
            current = TreeNode::with_children(value, None, boxed(current));
        }
        assert_eq!(max_depth(Some(&current)), 50);
    }

    #[test]
    fn zigzag_single_path_tree() {
        let mut current = TreeNode::leaf(20);
        for step in (1..20).rev() {
            current = if step % 2 == 0 {
                TreeNode::with_children(step, boxed(current), None)
            } else {
                TreeNode::with_children(step, None, boxed(current))
            };
        }
        assert_eq!(max_depth(Some(&current)), 20);
    }

    #[test]
    fn duplicate_negative_and_extreme_values_do_not_affect_depth() {
        let left = TreeNode::with_children(i32::MIN, leaf(0), leaf(i32::MIN));
        let right = TreeNode::with_children(
            i32::MAX,
            None,
            boxed(TreeNode::with_children(-1, leaf(i32::MAX), None)),
        );
        let root = TreeNode::with_children(0, boxed(left), boxed(right));
        assert_eq!(max_depth(Some(&root)), 4);
    }

    #[test]
    fn internal_deep_branch_off_right_child_left_subtree() {
        let right_left =
            TreeNode::with_children(6, boxed(TreeNode::with_children(7, None, leaf(8))), None);
        let right = TreeNode::with_children(3, boxed(right_left), leaf(9));
        let left = TreeNode::with_children(2, leaf(4), leaf(5));
        let root = TreeNode::with_children(1, boxed(left), boxed(right));
        assert_eq!(max_depth(Some(&root)), 5);
    }

    #[test]
    fn borrowed_tree_is_preserved_and_subtrees_can_be_measured() {
        let left = TreeNode::with_children(2, leaf(4), None);
        let right =
            TreeNode::with_children(3, None, boxed(TreeNode::with_children(5, leaf(6), None)));
        let root = TreeNode::with_children(1, boxed(left), boxed(right));
        let original = root.clone();

        assert_eq!(max_depth(Some(&root)), 4);
        assert_eq!(max_depth(root.left.as_deref()), 2);
        assert_eq!(max_depth(root.right.as_deref()), 3);
        assert_eq!(root, original);
    }
}
