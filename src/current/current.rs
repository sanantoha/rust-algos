use crate::tree::TreeNode;

pub fn kth_smallest_element(root: TreeNode, k: i32) -> Option<i32> {
    None
}


#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::kth_smallest_element;


    #[test]
    fn it_kth_smallest_element() {

        let root = TreeNode::new(5,
                                 Some(Box::new(TreeNode::new(2,
                                                             Some(Box::new(TreeNode::leaf(1))),
                                                             Some(Box::new(TreeNode::leaf(3))))
                                 )), Some(Box::new(TreeNode::new(10,
                                                                 Some(Box::new(TreeNode::new(7,
                                                                                             Some(Box::new(TreeNode::leaf(6))),
                                                                                             Some(Box::new(TreeNode::leaf(8))))
                                                                 )),
                                                                 Some(Box::new(TreeNode::new(15,
                                                                                             Some(Box::new(TreeNode::leaf(14))),
                                                                                             Some(Box::new(TreeNode::leaf(17))))
                                                                 ))
            ))));

        assert_eq!(kth_smallest_element(root, 5), Some(6));
    }
}