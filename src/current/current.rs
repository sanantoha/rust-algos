use crate::tree::TreeNode;

pub fn binary_tree_diameter(root: &Option<Box<TreeNode>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::binary_tree_diameter;

    #[test]
    fn test_binary_tree_diameter() {

        let root = Some(
            Box::new(TreeNode::new(
                1,
                Some(Box::new(TreeNode::new(3,
                                            Some(Box::new(TreeNode::new(7,
                                                                        Some(Box::new(TreeNode::new(8,
                                                                                                    Some(Box::new(TreeNode::leaf(9))),
                                                                                                    None
                                                                        ))),
                                                                        None
                                            ))),
                                            Some(Box::new(TreeNode::new(4,
                                                                        None,
                                                                        Some(Box::new(TreeNode::new(5,
                                                                                                    None,
                                                                                                    Some(Box::new(TreeNode::leaf(6)))
                                                                        )))
                                            )))
                ))),
                Some(Box::new(TreeNode::leaf(2))),
            ))
        );

        assert_eq!(binary_tree_diameter(&root), 6);
    }
}