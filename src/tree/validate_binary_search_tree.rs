use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn validate(root: &Option<Box<TreeNode>>) -> bool {
    helper(root, i64::MIN, i64::MAX)
}

fn helper(root: &Option<Box<TreeNode>>, min_val: i64, max_val: i64) -> bool {
    if let Some(node) = root {
        if (node.val as i64) < min_val || (node.val as i64) >= max_val {
            return false;
        }
        helper(&node.left, min_val, node.val as i64) && helper(&node.right, node.val as i64, max_val)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use crate::tree::validate_binary_search_tree::validate;

    #[test]
    fn test_validate() {
        let root = Some(Box::new(TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::leaf(15))),
            ))),
        )));

        assert!(validate(&root));
    }


}