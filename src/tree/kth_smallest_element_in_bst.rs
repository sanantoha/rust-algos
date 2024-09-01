use std::collections::VecDeque;

use super::TreeNode;

// O(n) time | O(1) space
pub fn kth_smallest_element(root: TreeNode, k: i32) -> Option<i32> {

    let box_root = Box::new(root);
    let mut curr = Some(&box_root);
    let mut stack = VecDeque::new();

    let mut idx = 0;

    while !stack.is_empty() || curr.is_some() {
        while let Some(node) = curr.take() {
            stack.push_back(node);
            
            curr = node.left.as_ref();
        }

        if let Some(node) = stack.pop_back() {
            if idx == k - 1 {
                return Some(node.val);
            }

            curr = node.right.as_ref();
        }

        idx += 1;
    }
    
    None
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::kth_smallest_element;


    #[test]
    fn it_kth_smallest_element() {

        let root = TreeNode::new(5, 
            Some(Box::new(TreeNode::new(2, 
                Some(Box::new(TreeNode::leaf(1))), 
                Some(Box::new(TreeNode::leaf(3))))
            )), Some(Box::new(TreeNode::new(10, 
                Some(Box::new(TreeNode::new(7, 
                    Some(Box::new(TreeNode::leaf(6))), 
                    Some(Box::new(TreeNode::leaf(8))))
                )), 
                Some(Box::new(TreeNode::new(15, 
                    Some(Box::new(TreeNode::leaf(14))), 
                    Some(Box::new(TreeNode::leaf(17))))
                ))
            ))));

        assert_eq!(kth_smallest_element(root, 5), Some(6));
    }
}