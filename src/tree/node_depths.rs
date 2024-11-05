use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn node_depths_rec(root: &Option<Box<TreeNode>>) -> i32 {
    helper(root, 0)
}

fn helper(root: &Option<Box<TreeNode>>, depth: i32) -> i32 {
    if let Some(node) = root {
        return depth + helper(&node.left, depth + 1) + helper(&node.right, depth + 1);
    }
    0
}

// O(n) time | O(h) space
pub fn node_depths(root: &Option<Box<TreeNode>>) -> i32 {
    let mut sum_depth = 0;
    if let Some(node) = root {

        let mut stack = vec![(node, 0)];

        while let Some((node, depth)) = stack.pop() {

            sum_depth += depth;


            if let Some(left) = node.left.as_ref() {
                stack.push((left, depth + 1));
            }
            if let Some(right) = node.right.as_ref() {
                stack.push((right, depth + 1));
            }
        }
    }
    sum_depth
}

#[cfg(test)]
mod tests {
    use crate::tree::node_depths::{node_depths, node_depths_rec};
    use crate::tree::TreeNode;

    #[test]
    fn test_node_depths_rec() {
        let root = create_tree();

        assert_eq!(node_depths_rec(&root), 16)
    }

    #[test]
    fn test_node_depths() {
        let root = create_tree();

        assert_eq!(node_depths(&root), 16)
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(
            1,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::new(
                    4,
                    Some(Box::new(TreeNode::leaf(8))),
                    Some(Box::new(TreeNode::leaf(9))),
                ))),
                Some(Box::new(TreeNode::leaf(5))),
            ))),
            Some(Box::new(TreeNode::new(
                3,
                Some(Box::new(TreeNode::leaf(6))),
                Some(Box::new(TreeNode::leaf(7))),
            ))),
        )));

        root
    }
}