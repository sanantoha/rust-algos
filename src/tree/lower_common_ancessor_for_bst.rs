use crate::tree::TreeNode;

// O(h) time | O(h) space
pub fn lower_common_ancestor<'a>(root: &'a Option<Box<TreeNode>>, p: &'a Option<Box<TreeNode>>, q: &'a Option<Box<TreeNode>>) -> &'a Option<Box<TreeNode>> {
    if let Some(node) = root {
        if let (Some(pp), Some(qq)) = (p, q) {
            if node.val < pp.val && node.val < qq.val {
                return lower_common_ancestor(&node.right, p, q);
            } else if node.val > pp.val && node.val > qq.val {
                return lower_common_ancestor(&node.left, p, q);
            } else {
                return root;
            }
        }
    }
    &None
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::lower_common_ancestor;

    #[test]
    fn test_lower_common_ancestor() {
        let root = Some(Box::new(TreeNode::new(
            6,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(0))),
                Some(Box::new(TreeNode::new(
                    4,
                    Some(Box::new(TreeNode::leaf(3))),
                    Some(Box::new(TreeNode::leaf(5))),
                ))),
            ))),
            Some(Box::new(TreeNode::new(
                8,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::leaf(9))),
            ))),
        )));

        let p = Some(Box::new(TreeNode::leaf(0)));
        let q = Some(Box::new(TreeNode::leaf(5)));

        let exp_tree = Some(Box::new(TreeNode::new(
            2,
            Some(Box::new(TreeNode::leaf(0))),
            Some(Box::new(TreeNode::new(
                4,
                Some(Box::new(TreeNode::leaf(3))),
                Some(Box::new(TreeNode::leaf(5))),
            ))),
        )));

        let res = lower_common_ancestor(&root, &p, &q);
        println!("{:?}", res);

        assert_eq!(res, &exp_tree);
    }

    #[test]
    fn test_lower_common_ancestor_case1() {
        let root = Some(Box::new(TreeNode::new(2, Some(Box::new(TreeNode::leaf(1))), None)));
        let p = Some(Box::new(TreeNode::leaf(2)));
        let q = Some(Box::new(TreeNode::leaf(1)));

        let res = lower_common_ancestor(&root, &p, &q);
        println!("{:?}", res);

        assert_eq!(res, &root);
    }
}