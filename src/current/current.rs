use crate::tree::TreeNode;

pub fn is_balanced(root: &Option<Box<TreeNode>>) -> bool {
    false
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