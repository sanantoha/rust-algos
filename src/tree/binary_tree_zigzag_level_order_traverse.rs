use std::collections::VecDeque;
use crate::tree::TreeNode;

// O(n) time | O(n) space
pub fn zig_zag(root: &Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    if let Some(node) = root {
        let mut res = vec![];

        let mut queue = VecDeque::new();
        queue.push_back(node);
        let mut idx = 0;
        while !queue.is_empty() {
            let mut size = queue.len();

            let mut sub_res = vec![];
            idx += 1;

            while size > 0 {
                size -= 1;
                if let Some(curr) = queue.pop_front() {
                    sub_res.push(curr.val);

                    if let Some(left) = curr.left.as_ref() {
                        queue.push_back(left);
                    }
                    if let Some(right) = curr.right.as_ref() {
                        queue.push_back(right);
                    }
                }
            }

            if idx % 2 == 0 {
                sub_res.reverse();
            }

            res.push(sub_res);
        }

        res
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::zig_zag;

    #[test]
    fn test_zig_zag() {

        let root = Some(
            Box::new(TreeNode::new(5,
                Some(Box::new(TreeNode::new(2,
                    Some(Box::new(TreeNode::leaf(1))),
                    Some(Box::new(TreeNode::leaf(3)))
                ))),
               Some(Box::new(TreeNode::new(10,
                   Some(Box::new(TreeNode::leaf(7))),
                   Some(Box::new(TreeNode::new(15,
                       Some(Box::new(TreeNode::leaf(14))),
                       Some(Box::new(TreeNode::leaf(17)))
                   )))
               )))
            ))
        );

        assert_eq!(zig_zag(&root), vec![vec![5], vec![10, 2], vec![1, 3, 7, 15], vec![17, 14]]);
    }
}