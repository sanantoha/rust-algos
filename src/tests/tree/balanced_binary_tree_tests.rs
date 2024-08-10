#[cfg(test)]
mod tests {
    #[test]
    fn test_balanced_binary_tree() {
        use super::super::super::super::tree::tree::TreeNode;
        use super::super::super::super::tree::balanced_binary_tree::is_balanced;

        let root = Some(Box::new(TreeNode::new(
            3,
            Some(Box::new(TreeNode::single(9))),
            Some(Box::new(TreeNode::new(
                20,
                Some(Box::new(TreeNode::single(15))),
                Some(Box::new(TreeNode::single(7))),
            ))),
        )));

        let root1 = Some(Box::new(
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

        assert!(is_balanced(&root));
        assert!(!is_balanced(&root1));
    }
}