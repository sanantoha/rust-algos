use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn pre_order(root: &Option<Box<TreeNode>>) -> Vec<i32> {

    let mut stack = vec![root];
    let mut res = vec![];

    while let Some(node_opt) = stack.pop() {
        if let Some(node) = node_opt {
            res.push(node.val);

            stack.push(&node.right);
            stack.push(&node.left);
        }
    }

    res
}

// O(n) time | O(h) space
pub fn in_order(root: &Option<Box<TreeNode>>) -> Vec<i32> {

    let mut stack = vec![];
    let mut res = vec![];

    let mut curr = root;

    while !stack.is_empty() || curr.is_some() {
        while let Some(node) = curr {
            stack.push(node);
            curr = &node.left;
        }

        if let Some(node) = stack.pop() {
            res.push(node.val);

            curr = &node.right;
        }
    }

    res
}

// O(n) time | O(h) space
pub fn post_order(root: &Option<Box<TreeNode>>) -> Vec<i32> {

    let mut fst = vec![root];
    let mut snd = vec![];

    let mut res = vec![];

    while let Some(node_opt) = fst.pop() {
        if let Some(node) = node_opt {
            snd.push(node);

            fst.push(&node.left);
            fst.push(&node.right);
        }
    }

    while let Some(node) = snd.pop() {
        res.push(node.val);
    }

    res
}


#[cfg(test)]
mod tests {

    use crate::tree::TreeNode;
    use super::{pre_order, in_order, post_order};

    #[test]
    pub fn test_dfs_pre_order() {
        assert_eq!(pre_order(&create_tree()), vec![5, 2, 1, 3, 8, 7, 9]);        
    }

    #[test]
    pub fn test_dfs_in_order() {
        assert_eq!(in_order(&create_tree()), vec![1, 2, 3, 5, 7, 8, 9]);        
    }

    #[test]
    pub fn test_dfs_post_order() {
        assert_eq!(post_order(&create_tree()), vec![1, 3, 2, 7, 9, 8, 5]);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        Some(
            Box::new(TreeNode::new(5,
                                Some(Box::new(TreeNode::new(2,
                                                            Some(Box::new(TreeNode::leaf(1))),
                                                            Some(Box::new(TreeNode::leaf(3)))))),
                                Some(Box::new(TreeNode::new(8,
                                                            Some(Box::new(TreeNode::leaf(7))),
                                                            Some(Box::new(TreeNode::leaf(9)))))),
            ))
        )
    }
}