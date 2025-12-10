use crate::tree::TreeNode;

pub fn zig_zag(root: &Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::zig_zag;

    #[test]
    fn test_zig_zag() {

        let root = Some(
            Box::new(TreeNode::new(5,
                                   Some(Box::new(TreeNode::new(2,
                                                               Some(Box::new(TreeNode::leaf(1))),
                                                               Some(Box::new(TreeNode::leaf(3)))
                                   ))),
                                   Some(Box::new(TreeNode::new(10,
                                                               Some(Box::new(TreeNode::leaf(7))),
                                                               Some(Box::new(TreeNode::new(15,
                                                                                           Some(Box::new(TreeNode::leaf(14))),
                                                                                           Some(Box::new(TreeNode::leaf(17)))
                                                               )))
                                   )))
            ))
        );

        assert_eq!(zig_zag(&root), vec![vec![5], vec![10, 2], vec![1, 3, 7, 15], vec![17, 14]]);
    }
}