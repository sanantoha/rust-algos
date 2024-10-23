use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn binary_tree_diameter(root: &Option<Box<TreeNode>>) -> i32 {
    helper(root).0
}

fn helper(root: &Option<Box<TreeNode>>) -> (i32, i32) {
    if let Some(node) = root {
        let (ld, lh) = helper(&node.left);
        let (rd, rh) = helper(&node.right);

        let h = 1 + lh.max(rh);
        let d = ld.max(rd).max(lh + rh);

        (d, h)
    } else {
        (0, 0)
    }
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

