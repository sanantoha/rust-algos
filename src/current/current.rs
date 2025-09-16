use crate::tree::TreeNode;

pub fn bfs(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::bfs;
    use crate::tree::TreeNode;

    #[test]
    pub fn test_bfs() {

        let root = Some(Box::new(TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::new(
                    15,
                    Some(Box::new(TreeNode::leaf(13))),
                    Some(Box::new(TreeNode::leaf(17))),
                ))),
            ))),
        )));

        assert_eq!(bfs(&root), vec![5, 2, 10, 1, 3, 7, 15, 13, 17]);
    }
}