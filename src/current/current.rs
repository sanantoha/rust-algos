use crate::tree::TreeNode;

pub fn sorted_array_to_bst(_arr: &[i32]) -> Option<Box<TreeNode>> {
    None
}

#[cfg(test)]
mod tests {
    use super::sorted_array_to_bst;
    use crate::tree::TreeNode;

    #[test]
    fn it_sorted_array_to_bst() {
        let arr: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let tree = sorted_array_to_bst(arr);
        println!("{:?}", tree);

        let exp_tree = Some(Box::new(TreeNode::new(5,
                                                   Some(Box::new(TreeNode::new(2,
                                                                               Some(Box::new(TreeNode::leaf(1))),
                                                                               Some(Box::new(TreeNode::new(3,
                                                                                                           None,
                                                                                                           Some(Box::new(TreeNode::leaf(4)))
                                                                               )))
                                                   ))),
                                                   Some(Box::new(TreeNode::new(8,
                                                                               Some(Box::new(TreeNode::new(6,
                                                                                                           None,
                                                                                                           Some(Box::new(TreeNode::leaf(7)))
                                                                               ))),
                                                                               Some(Box::new(TreeNode::new(9,
                                                                                                           None,
                                                                                                           Some(Box::new(TreeNode::leaf(10)))
                                                                               ))),
                                                   ))),
        )));

        assert_eq!(tree, exp_tree);
    }
}