use crate::tree::TreeNode;

pub fn validate(root: &Option<Box<TreeNode>>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::validate;

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