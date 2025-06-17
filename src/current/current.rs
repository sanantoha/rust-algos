use crate::tree::TreeNode;

pub fn level_order(root: Box<TreeNode>) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::level_order;


    #[test]
    fn it_level_order() {
        let root = Box::new(TreeNode::new(5,
                                          Some(Box::new(TreeNode::new(2,
                                                                      Some(Box::new(TreeNode::leaf(1))),
                                                                      Some(Box::new(TreeNode::leaf(3)))))),
                                          Some(Box::new(TreeNode::new(10,
                                                                      Some(Box::new(TreeNode::new(7,
                                                                                                  Some(Box::new(TreeNode::leaf(6))),
                                                                                                  Some(Box::new(TreeNode::leaf(8)))))),
                                                                      Some(Box::new(TreeNode::new(15,
                                                                                                  Some(Box::new(TreeNode::leaf(14))),
                                                                                                  Some(Box::new(TreeNode::leaf(17)))))))))),
        );

        let res = level_order(root);
        println!("{res:?}");
        assert_eq!(res, vec![vec![5], vec![2, 10], vec![1, 3, 7, 15], vec![6, 8, 14, 17]]);
    }
}