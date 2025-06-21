use crate::tree::TreeNode;

pub fn find_mode(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    vec![]
}


#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::find_mode;

    #[test]
    fn test_find_mode() {

        let root = Some(Box::new(TreeNode::new(1,
                                               None,
                                               Some(Box::new(TreeNode::new(2,
                                                                           Some(Box::new(TreeNode::leaf(2))),
                                                                           None
                                               )))
        )));

        assert_eq!(find_mode(&root), vec![2]);
    }

    #[test]
    fn test_find_mode_case1() {

        let root = Some(Box::new(TreeNode::leaf(0)));

        assert_eq!(find_mode(&root), vec![0]);
    }

    #[test]
    fn test_find_mode_case2() {

        let root = Some(Box::new(TreeNode::new(5,
                                               Some(Box::new(TreeNode::new(3,
                                                                           Some(Box::new(TreeNode::leaf(1))),
                                                                           Some(Box::new(TreeNode::leaf(3)))
                                               ))),
                                               Some(Box::new(TreeNode::new(7,
                                                                           Some(Box::new(TreeNode::leaf(5))),
                                                                           Some(Box::new(TreeNode::leaf(7)))
                                               )))
        )));

        assert_eq!(find_mode(&root), vec![3, 5, 7]);
    }
}