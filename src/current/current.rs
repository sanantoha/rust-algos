use crate::tree::TreeNode;


pub fn left_view(root: Box<TreeNode>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::left_view;
    use crate::tree::TreeNode;

    #[test]
    fn it_left_view() {
        let root = Box::new(TreeNode::new(1,
                                          Some(Box::new(TreeNode::leaf(2))),
                                          Some(Box::new(TreeNode::new(3,
                                                                      Some(Box::new(TreeNode::new(4,
                                                                                                  Some(Box::new(TreeNode::leaf(5))),
                                                                                                  Some(Box::new(TreeNode::new(6,
                                                                                                                              None,
                                                                                                                              Some(Box::new(TreeNode::leaf(7))))))))),
                                                                      Some(Box::new(TreeNode::leaf(8)))))),
        ));

        let res = left_view(root);
        println!("{res:?}");
        assert_eq!(res, vec![1, 2, 4, 5, 7]);
    }
}