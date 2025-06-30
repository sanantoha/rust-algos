use crate::tree::TreeNode;

pub fn reconstruct_bst(arr: &[i32]) -> Option<Box<TreeNode>> {
    None
}

pub fn reconstruct_bst1(arr: &[i32]) -> Option<Box<TreeNode>> {
    None
}


#[cfg(test)]
mod tests {
    use super::{reconstruct_bst, reconstruct_bst1};
    use crate::tree::TreeNode;

    const ARR: &[i32] = &[10, 4, 2, 1, 3, 17, 19, 18];

    #[test]
    fn test_reconstruct_bst() {
        let exp_tree = create_exp_tree();

        let res = reconstruct_bst(ARR);
        assert_eq!(res, exp_tree)
    }

    #[test]
    fn test_reconstruct_bst1() {
        let exp_tree = create_exp_tree();

        let res = reconstruct_bst1(ARR);
        assert_eq!(res, exp_tree)
    }

    fn create_exp_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(10,
                                               Some(Box::new(TreeNode::new(4,
                                                                           Some(Box::new(TreeNode::new(2,
                                                                                                       Some(Box::new(TreeNode::leaf(1))),
                                                                                                       Some(Box::new(TreeNode::leaf(3)))
                                                                           ))),
                                                                           None
                                               ))),
                                               Some(Box::new(TreeNode::new(17,
                                                                           None,
                                                                           Some(Box::new(TreeNode::new(19,
                                                                                                       Some(Box::new(TreeNode::leaf(18))),
                                                                                                       None
                                                                           )))
                                               ))),
        )));

        root
    }
}