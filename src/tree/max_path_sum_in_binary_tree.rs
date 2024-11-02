use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn max_path_sum(root: &Option<Box<TreeNode>>) -> i32 {
    helper(root).0
}

fn helper(root: &Option<Box<TreeNode>>) -> (i32, i32) {
    if let Some(node) = root {
        let (max_path_left, max_sum_branch_left) = helper(&node.left);
        let (max_path_right, max_sum_branch_right) = helper(&node.right);

        let max_sum_child_as_branch = max_sum_branch_left.max(max_sum_branch_right);
        let max_sum_as_branch = node.val.max(node.val + max_sum_child_as_branch);

        let max_sum_as_root_node = max_sum_as_branch.max(max_sum_branch_left + node.val + max_sum_branch_right);
        let max_path_sum = max_path_left.max(max_path_right).max(max_sum_as_root_node);
        (max_path_sum, max_sum_as_branch)
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::max_path_sum;

    #[test]
    fn test_max_path_sum() {
        let root = TreeNode::new(1,
             Some(Box::new(TreeNode::new(2,
                 Some(Box::new(TreeNode::leaf(4))),
                 Some(Box::new(TreeNode::leaf(5)))
             ))),
             Some(Box::new(TreeNode::new(3,
                 Some(Box::new(TreeNode::leaf(6))),
                 Some(Box::new(TreeNode::leaf(7)))
             )))
        );

        assert_eq!(max_path_sum(&Some(Box::new(root))), 18);
    }
}