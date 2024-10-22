use std::collections::VecDeque;
use crate::tree::TreeNode;

// O(l1 + l2) time | O(l1 + l2) space
pub fn get_all_elements(root1: &Option<Box<TreeNode>>, root2: &Option<Box<TreeNode>>) -> Vec<i32> {

    let mut stack1 = VecDeque::new();
    let mut curr1 = root1.as_ref();

    let mut stack2 = VecDeque::new();
    let mut curr2 = root2.as_ref();

    let mut res = vec![];

    while curr1.is_some() || !stack1.is_empty() || curr2.is_some() || !stack2.is_empty() {
        while let Some(node1) = curr1.take() {
            stack1.push_back(node1);
            curr1 = node1.left.as_ref();
        }

        while let Some(node2) = curr2.take() {
            stack2.push_back(node2);
            curr2 = node2.left.as_ref();
        }

        if stack2.is_empty() || !stack1.is_empty() && stack1.back().unwrap().val <= stack2.back().unwrap().val {
            if let Some(n1) = stack1.pop_back() {
                res.push(n1.val);
                curr1 = n1.right.as_ref();
            }
        } else {
            if let Some(n2) = stack2.pop_back() {
                res.push(n2.val);
                curr2 = n2.right.as_ref();
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::get_all_elements;

    #[test]
    fn test_get_all_elements() {
        let root1 = Some(
            Box::new(TreeNode::new(9,
                                   Some(Box::new(TreeNode::leaf(2))),
                                   Some(Box::new(TreeNode::new(12,
                                                               Some(Box::new(TreeNode::leaf(11))),
                                                               Some(Box::new(TreeNode::leaf(15)))))),
            ))
        );

        let root2 = Some(
            Box::new(TreeNode::new(10,
                                   Some(Box::new(TreeNode::new(5,
                                                               Some(Box::new(TreeNode::new(3,
                                                                    Some(Box::new(TreeNode::leaf(1))),
                                                                    Some(Box::new(TreeNode::leaf(4)))
                                                               ))),
                                                               Some(Box::new(TreeNode::leaf(6))))
                                   )),
                                   Some(Box::new(TreeNode::leaf(16))))
            )
        );

        assert_eq!(get_all_elements(&root1, &root2), vec![1, 2, 3, 4, 5, 6, 9, 10, 11, 12, 15, 16]);
    }
}