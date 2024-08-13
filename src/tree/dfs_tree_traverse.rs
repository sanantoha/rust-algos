use std::collections::LinkedList;
use crate::tree::tree::TreeNode;

// O(n) time | O(h) space
pub fn pre_order(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    if let Some(node) = root {
        let mut stack = LinkedList::new();
        stack.push_back(node);

        let mut res = vec![];

        while !stack.is_empty() {
            if let Some(curr) = stack.pop_back() {

                res.push(curr.val);

                if let Some(right) = &curr.right {
                    stack.push_back(&right);
                }
                if let Some(left) = &curr.left {
                    stack.push_back(&left);
                }
            }
        }

        return res;
    }

    return vec![];
}

// O(n) time | O(h) space
pub fn in_order(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    if let Some(node) = root {
        let mut stack = LinkedList::new();
        let mut curr = Some(node);

        let mut res = vec![];

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push_back(node);
                curr = node.left.as_ref();
            }

            if let Some(node) = stack.pop_back() {
                res.push(node.val);

                curr = node.right.as_ref();
            }
        }

        return res;
    }

    return vec![];
}

// O(n) time | O(h) space
pub fn post_order(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    if let Some(node) = root {
        let mut stack1 = LinkedList::new();
        let mut stack2 = LinkedList::new();

        stack1.push_back(node);

        while !stack1.is_empty() {
            if let Some(curr) = stack1.pop_back() {
                stack2.push_back(curr);

                if let Some(left) = &curr.left {
                    stack1.push_back(left);
                }
                if let Some(right) = &curr.right {
                    stack1.push_back(right);
                }
            }
        }

        let mut res = vec![];

        while !stack2.is_empty() {
            if let Some(node) = stack2.pop_back() {
                res.push(node.val);
            }
        }
        return res;
    }
    return vec![];
}

#[test]
pub fn test_pre_order() {

    let root = Some(
        Box::new(TreeNode::new(5,
                               Some(Box::new(TreeNode::new(2,
                                                           Some(Box::new(TreeNode::single(1))),
                                                           Some(Box::new(TreeNode::single(3)))))),
                               Some(Box::new(TreeNode::new(8,
                                                           Some(Box::new(TreeNode::single(7))),
                                                           Some(Box::new(TreeNode::single(9)))))),
        ))
    );

    assert_eq!(pre_order(&root), vec![5, 2, 1, 3, 8, 7, 9]);
    assert_eq!(in_order(&root), vec![1, 2, 3, 5, 7, 8, 9]);
    assert_eq!(post_order(&root), vec![1, 3, 2, 7, 9, 8, 5]);
}