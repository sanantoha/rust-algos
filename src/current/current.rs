use crate::tree::TreeNode;

pub fn get_minimum_difference(root: &Option<Box<TreeNode>>) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::get_minimum_difference;
    use crate::tree::TreeNode;

    #[test]
    fn test_get_minimum_difference() {
        let root = Some(Box::new(TreeNode::new(
            4,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::leaf(6))),
        )));

        assert_eq!(get_minimum_difference(&root), 1);
    }

    #[test]
    fn test_minimum_absolute_difference_case1() {
        let root = Some(Box::new(TreeNode::new(
            5,
            Some(Box::new(TreeNode::leaf(0))),
            Some(Box::new(TreeNode::new(
                48,
                Some(Box::new(TreeNode::leaf(12))),
                Some(Box::new(TreeNode::leaf(50))),
            ))),
        )));

        assert_eq!(get_minimum_difference(&root), 2);
    }
}