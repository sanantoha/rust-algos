use crate::tree::TreeNode;

pub fn is_same_tree(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
    false
}

pub fn is_same_tree_iter(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::{is_same_tree, is_same_tree_iter};

    #[test]
    fn test_is_same_tree() {
        let t1 = create_tree1();
        let t2 = create_tree1();
        assert!(is_same_tree(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_iter() {
        let t1 = create_tree1();
        let t2 = create_tree1();
        assert!(is_same_tree_iter(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_neg() {
        let t1 = create_tree1();
        let t2 = create_tree2();
        assert!(!is_same_tree(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_iter_neg() {
        let t1 = create_tree1();
        let t2 = create_tree2();
        assert!(!is_same_tree_iter(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_case1() {
        let t1 = create_tree3();
        let t2 = create_tree31();

        assert!(!is_same_tree(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_iter_case1() {
        let t1 = create_tree3();
        let t2 = create_tree31();
        assert!(!is_same_tree_iter(&t1, &t2));
    }

    fn create_tree1() -> Option<Box<TreeNode>> {
        let tree = TreeNode::new(
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
        );

        Some(Box::new(tree))
    }

    fn create_tree2() -> Option<Box<TreeNode>> {
        let tree = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(444444444))),
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::leaf(15))),
            ))),
        );

        Some(Box::new(tree))
    }

    fn create_tree3() -> Option<Box<TreeNode>> {
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

    fn create_tree31() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(3))),
                None,
            ))),
            Some(Box::new(TreeNode::new(
                2,
                None,
                Some(Box::new(TreeNode::leaf(3))),
            ))),
        );
        Some(Box::new(root))
    }
}