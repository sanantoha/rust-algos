use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn evaluate_expression_tree(tree: &Option<Box<TreeNode>>) -> i32 {
    if let Some(node) = tree {
        if node.val > 0 {
            return node.val;
        }

        let lv = evaluate_expression_tree(&node.left);
        let rv = evaluate_expression_tree(&node.right);

        if node.val == -1 {
            return lv + rv;
        } else if node.val == -2 {
            return lv - rv;
        } else if node.val == -3 {
            return lv / rv;
        } else if node.val == -4 {
            return lv * rv;
        } else {
            return node.val;
        }
    }
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