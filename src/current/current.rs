use crate::tree::TreeNode;

pub fn merge_binary_trees(left: &mut Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    None
}

pub fn merge_binary_trees_iter(left: &mut Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    None
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::tree::TreeNode;
    use super::{merge_binary_trees, merge_binary_trees_iter};

    #[test]
    fn test_merge_binary_trees() {
        let mut left = create_left_tree();
        let right = create_right_tree();
        let exp_tree = create_exp_tree();

        assert_equal(merge_binary_trees(&mut left, &right), exp_tree);
    }

    #[test]
    fn test_merge_binary_trees_iter() {
        let mut left = create_left_tree();
        let right = create_right_tree();
        let exp_tree = create_exp_tree();

        assert_equal(merge_binary_trees_iter(&mut left, &right), exp_tree);
    }

    fn create_left_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(1,
                                 Some(Box::new(TreeNode::new(3,
                                                             Some(Box::new(TreeNode::leaf(7))),
                                                             Some(Box::new(TreeNode::leaf(4)))
                                 ))),
                                 Some(Box::new(TreeNode::leaf(2)))
        );

        Some(Box::new(root))
    }

    fn create_right_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(1,
                                 Some(Box::new(TreeNode::new(5,
                                                             Some(Box::new(TreeNode::leaf(2))),
                                                             None
                                 ))),
                                 Some(Box::new(TreeNode::new(9,
                                                             Some(Box::new(TreeNode::leaf(7))),
                                                             Some(Box::new(TreeNode::leaf(6)))
                                 )))
        );

        Some(Box::new(root))
    }

    fn create_exp_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(2,
                                 Some(Box::new(TreeNode::new(8,
                                                             Some(Box::new(TreeNode::leaf(9))),
                                                             Some(Box::new(TreeNode::leaf(4)))
                                 ))),
                                 Some(Box::new(TreeNode::new(11,
                                                             Some(Box::new(TreeNode::leaf(7))),
                                                             Some(Box::new(TreeNode::leaf(6)))
                                 )))
        );

        Some(Box::new(root))
    }
}