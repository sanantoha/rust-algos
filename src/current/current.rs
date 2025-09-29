use crate::tree::TreeNode;

pub fn get_all_elements(root1: &Option<Box<TreeNode>>, root2: &Option<Box<TreeNode>>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::get_all_elements;

    #[test]
    fn test_get_all_elements() {
        let root1 = Some(
            Box::new(TreeNode::new(9,
                                   Some(Box::new(TreeNode::leaf(2))),
                                   Some(Box::new(TreeNode::new(12,
                                                               Some(Box::new(TreeNode::leaf(11))),
                                                               Some(Box::new(TreeNode::leaf(15)))))),
            ))
        );

        let root2 = Some(
            Box::new(TreeNode::new(10,
                                   Some(Box::new(TreeNode::new(5,
                                                               Some(Box::new(TreeNode::new(3,
                                                                                           Some(Box::new(TreeNode::leaf(1))),
                                                                                           Some(Box::new(TreeNode::leaf(4)))
                                                               ))),
                                                               Some(Box::new(TreeNode::leaf(6))))
                                   )),
                                   Some(Box::new(TreeNode::leaf(16))))
            )
        );

        assert_eq!(get_all_elements(&root1, &root2), vec![1, 2, 3, 4, 5, 6, 9, 10, 11, 12, 15, 16]);
    }
}