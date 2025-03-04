use std::collections::VecDeque;
use crate::tree::TreeNode;

// O(n) time | O(n) space
pub fn bfs(root: &Option<Box<TreeNode>>) -> Vec<i32> {

    let mut res = vec![];

    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);

        while let Some(node) = queue.pop_front() {
            res.push(node.val);

            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }
    }

    res
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