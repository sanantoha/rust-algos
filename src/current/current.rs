use crate::tree::TreeNode;


pub fn reverse(root: &mut Option<Box<TreeNode>>) {

}


pub fn reverse_iter(root: &mut Option<Box<TreeNode>>) {

}

#[cfg(test)]
mod tests {
    use super::{reverse, reverse_iter};
    use crate::tree::TreeNode;

    #[test]
    fn test_reverse() {
        let mut root = create_tree();
        let exp_tree = create_exp_tree();

        reverse(&mut root);

        assert_eq!(root, exp_tree);
    }

    #[test]
    fn test_reverse_iter() {
        let mut root = create_tree();
        let exp_tree = create_exp_tree();

        reverse_iter(&mut root);

        assert_eq!(root, exp_tree);
    }

    fn create_exp_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::new(15,
                                            Some(Box::new(TreeNode::leaf(17))),
                                            Some(Box::new(TreeNode::leaf(14))),
                ))),
                Some(Box::new(TreeNode::leaf(7))),
            ))),
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(3))),
                Some(Box::new(TreeNode::leaf(1))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::new(
                    15,
                    Some(Box::new(TreeNode::leaf(14))),
                    Some(Box::new(TreeNode::leaf(17))),
                ))),
            ))),
        );

        Some(Box::new(root))
    }
}