use crate::tree::TreeNode;

pub fn evaluate_expression_tree(root: &Option<Box<TreeNode>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::evaluate_expression_tree;

    #[test]
    fn test_evaluate_expression_tree() {

        let root = Some(Box::new(TreeNode::new(-1,
                                               Some(Box::new(TreeNode::new(-2,
                                                                           Some(Box::new(TreeNode::new(-4,
                                                                                                       Some(Box::new(TreeNode::leaf(3))),
                                                                                                       Some(Box::new(TreeNode::leaf(2)))
                                                                           ))),
                                                                           Some(Box::new(TreeNode::leaf(2)))
                                               ))),
                                               Some(Box::new(TreeNode::new(-3,
                                                                           Some(Box::new(TreeNode::leaf(8))),
                                                                           Some(Box::new(TreeNode::leaf(3)))
                                               )))
        )));

        assert_eq!(evaluate_expression_tree(&root), 6);
    }
}