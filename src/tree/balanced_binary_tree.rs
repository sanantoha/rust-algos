use crate::tree::tree::TreeNode;

// O(n) time | O(h) space
pub fn is_balanced(root: &Option<Box<TreeNode>>) -> bool {
    return helper(root).0
}

fn helper(root: &Option<Box<TreeNode>>) -> (bool, usize) {
    if let Some(node) = root {
        let (lb, lh) = helper(&node.left);
        let (rb, rh) = helper(&node.right);

        let is_balanced_val = lb && rb && lh.abs_diff(rh) <= 1;
        let height = lh.max(rh) + 1;

        return (is_balanced_val, height)
    }

    return (true, 0)
}