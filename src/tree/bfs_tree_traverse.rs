use std::collections::LinkedList;
use crate::tree::tree::TreeNode;

// O(n) time | O(n) space
pub fn bfs(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    if let Some(node) = root {

        // let mut queue = VecDeque::new(); // based on growable ring buffer
        let mut queue = LinkedList::new();
        queue.push_back(node);

        let mut res = vec![];

        while !queue.is_empty() {
            if let Some(curr) = queue.pop_front() {
                res.push(curr.val);

                if let Some(left) = &curr.left {
                    queue.push_back(left);
                }

                if let Some(right) = &curr.right {
                    queue.push_back(right);
                }
            }
        }
        return res;
    } else {
        return vec![];
    }
}

#[test]
pub fn test_bfs() {

    let root = Some(Box::new(TreeNode::new(
        5,
        Some(Box::new(TreeNode::new(
            2,
            Some(Box::new(TreeNode::single(1))),
            Some(Box::new(TreeNode::single(3))),
        ))),
        Some(Box::new(TreeNode::new(
            10,
            Some(Box::new(TreeNode::single(7))),
            Some(Box::new(TreeNode::new(
                15,
                Some(Box::new(TreeNode::single(13))),
                Some(Box::new(TreeNode::single(17))),
            ))),
        ))),
    )));

    assert_eq!(bfs(&root), vec![5, 2, 10, 1, 3, 7, 15, 13, 17]);
}