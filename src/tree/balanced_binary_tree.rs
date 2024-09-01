use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn is_balanced(root: &Option<Box<TreeNode>>) -> bool {
    helper(root).0
}

fn helper(root: &Option<Box<TreeNode>>) -> (bool, usize) {
    if let Some(node) = root {
        let (lb, lh) = helper(&node.left);
        let (rb, rh) = helper(&node.right);

        let is_balanced_val = lb && rb && lh.abs_diff(rh) <= 1;
        let height = lh.max(rh) + 1;

        return (is_balanced_val, height)
    }

    (true, 0)
}

#[cfg(test)]
mod tests {

    use super::is_balanced;
    use crate::tree::TreeNode;

    #[test]
    fn test_balanced_binary_tree() {

        let root = Some(Box::new(TreeNode::new(
            3,
            Some(Box::new(TreeNode::leaf(9))),
            Some(Box::new(TreeNode::new(
                20,
                Some(Box::new(TreeNode::leaf(15))),
                Some(Box::new(TreeNode::leaf(7))),
            ))),
        )));        

        assert!(is_balanced(&root));
    }

    #[test]
    pub fn test_not_balanced_tree() {

        let root = Some(Box::new(
            TreeNode::new(
                1,
                Some(Box::new(TreeNode::new(
                    2,
                    Some(Box::new(TreeNode::new(
                        3,
                        Some(Box::new(TreeNode::new(4, None, None))),
                        Some(Box::new(TreeNode::new(4, None, None))),
                    ))),
                    Some(Box::new(TreeNode::new(3, None, None))),
                ))),
                Some(Box::new(TreeNode::new(2, None, None))),
            )
        ));

        assert!(!is_balanced(&root));
    }
}