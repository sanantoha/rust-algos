use crate::tree::TreeNode;

pub fn is_symmetric_rec(root: &Option<Box<TreeNode>>) -> bool {
    false
}


pub fn is_symmetric(root: &Option<Box<TreeNode>>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{is_symmetric, is_symmetric_rec};
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