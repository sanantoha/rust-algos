use crate::tree::TreeNode;

pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Box<TreeNode>> {
    None
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::build_tree;

    #[test]
    fn test_build_tree() {
        let preorder: &[i32] = &[3, 9, 20, 15, 7];
        let inorder: &[i32] = &[9, 3, 15, 20, 7];

        let tree = build_tree(preorder, inorder);

        /*
                     3
                  /    \
                9      20
                     /   \
                   15     7
         */

        let exp_tree = Some(
            Box::new(TreeNode::new(3,
                                   Some(Box::new(TreeNode::leaf(9))),
                                   Some(Box::new(TreeNode::new(20,
                                                               Some(Box::new(TreeNode::leaf(15))),
                                                               Some(Box::new(TreeNode::leaf(7))),
                                   ))),
            ))
        );

        assert_eq!(tree, exp_tree);
    }
}