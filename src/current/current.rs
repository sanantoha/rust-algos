use crate::tree::TreeNode;

pub fn find_closest_value_in_bst_rec(root: &Option<Box<TreeNode>>, target: i32) -> Option<i32> {
    None
}

pub fn find_closest_value_in_bst(root: &Option<Box<TreeNode>>, target: i32) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::{find_closest_value_in_bst_rec, find_closest_value_in_bst};

    #[test]
    fn test_find_closest_value_in_bst_rec() {
        let root = create_tree();

        assert_eq!(find_closest_value_in_bst_rec(&root, 12), Some(13));
    }

    #[test]
    fn test_find_closest_value_in_bst() {
        let root = create_tree();

        assert_eq!(find_closest_value_in_bst(&root, 12), Some(13));
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            10,
            Some(Box::new(TreeNode::new(
                5,
                Some(Box::new(TreeNode::new(
                    2,
                    Some(Box::new(TreeNode::leaf(1))),
                    None,
                ))),
                Some(Box::new(TreeNode::leaf(5))),
            ))),
            Some(Box::new(TreeNode::new(
                15,
                Some(Box::new(TreeNode::new(
                    13,
                    None,
                    Some(Box::new(TreeNode::leaf(14))),
                ))),
                Some(Box::new(TreeNode::leaf(22))),
            ))),
        );

        Some(Box::new(root))
    }
}