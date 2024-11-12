use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn is_symmetric_rec(root: &Option<Box<TreeNode>>) -> bool {
    if let Some(node) = root {
        helper(&node.left, &node.right)
    } else {
        true
    }
}

fn helper(left: &Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> bool {
    if let (Some(left_node), Some(right_node)) = (left, right) {

        left_node.val == right_node.val &&
            helper(&left_node.left, &right_node.right) &&
            helper(&left_node.right, &right_node.left)
    } else if left.is_none() && right.is_none() {
        true
    } else {
        false
    }
}

// O(n) time | O(h) space
pub fn is_symmetric(root: &Option<Box<TreeNode>>) -> bool {
    if let Some(node) = root {

        if node.left.is_none() && node.right.is_none() {
            return true;
        }
        if let (Some(left_node), Some(right_node)) = (&node.left, &node.right) {
            let mut stack = vec![left_node, right_node];

            while let Some(curr1) = stack.pop() {
                if let Some(curr2) = stack.pop() {

                    if curr1.val != curr2.val {
                        return false;
                    }

                    if let (Some(left1), Some(right2)) = (&curr1.left, &curr2.right) {
                        stack.push(left1);
                        stack.push(right2);
                    } else if !(curr1.left.is_none() && curr2.right.is_none()) {
                        return false;
                    }

                    if let (Some(right1), Some(left2)) = (&curr1.right, &curr2.left) {
                        stack.push(right1);
                        stack.push(left2);
                    } else if !(curr1.right.is_none() && curr2.left.is_none()) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::simmetric_tree::{is_symmetric, is_symmetric_rec};
    use crate::tree::TreeNode;

    #[test]
    fn test_is_symmetric() {
        let root = create_tree();

        assert!(is_symmetric(&root));
    }

    #[test]
    fn test_is_symmetric_rec() {
        let root = create_tree();

        assert!(is_symmetric_rec(&root));
    }

    #[test]
    fn test_is_symmetric_case1() {
        let root = create_tree1();

        assert!(!is_symmetric(&root));
    }

    #[test]
    fn test_is_symmetric_rec_case1() {
        let root = create_tree1();

        assert!(!is_symmetric_rec(&root));
    }

    #[test]
    fn test_is_symmetric_case2() {
        let root = create_tree2();

        assert!(!is_symmetric(&root));
    }

    #[test]
    fn test_is_symmetric_rec_case2() {
        let root = create_tree2();

        assert!(!is_symmetric_rec(&root));
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(3))),
                Some(Box::new(TreeNode::leaf(4))),
            ))),
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(4))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree1() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            Some(Box::new(TreeNode::new(
                2,
                None,
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::new(
                2,
                None,
                Some(Box::new(TreeNode::leaf(3))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree2() -> Option<Box<TreeNode>> {
        // TreeNode root2 = new TreeNode(2, new TreeNode(1), new TreeNode(3));
        let root = TreeNode::new(2,
                                 Some(Box::new(TreeNode::leaf(1))),
                                 Some(Box::new(TreeNode::leaf(3)))
        );
        Some(Box::new(root))
    }
}