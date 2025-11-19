use crate::tree::TreeNode;

pub fn find_tilt(root: &Option<Box<TreeNode>>) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::find_tilt;

    #[test]
    fn it_find_tilt() {
        let root = Some(Box::new(TreeNode::new(1, Some(Box::new(TreeNode::leaf(2))), Some(Box::new(TreeNode::leaf(3))))));

        assert_eq!(find_tilt(&root), 1);
    }

    #[test]
    fn it_find_tilt1() {
        let root = Some(Box::new(TreeNode::new(
            4,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(3))),
                Some(Box::new(TreeNode::leaf(5))),
            ))),
            Some(Box::new(TreeNode::new(
                9,
                None,
                Some(Box::new(TreeNode::leaf(7))),
            ))),
        )));

        assert_eq!(find_tilt(&root), 15);
    }

    #[test]
    fn it_find_titl2() {
        let root = Some(Box::new(TreeNode::new(
            21,
            Some(Box::new(TreeNode::new(
                7,
                Some(Box::new(TreeNode::new(
                    1,
                    Some(Box::new(TreeNode::leaf(3))),
                    Some(Box::new(TreeNode::leaf(3))),
                ))),
                Some(Box::new(TreeNode::leaf(1))),
            ))),
            Some(Box::new(TreeNode::new(
                14,
                Some(Box::new(TreeNode::leaf(2))),
                Some(Box::new(TreeNode::leaf(2))),
            ))),
        )));

        assert_eq!(find_tilt(&root), 9);
    }
}