use crate::tree::tree::TreeNode;

// O(n) time | O(h) space
pub fn pre_order_rec(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];
    pre_order_rec_helper(root, &mut res);
    return res
}

fn pre_order_rec_helper(root: &Option<Box<TreeNode>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        res.push(node.val);
        pre_order_rec_helper(&node.left, res);
        pre_order_rec_helper(&node.right, res)
    }
}

// O(n) time | O(h) space
pub fn in_order_rec(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];
    in_order_rec_helper(root, &mut res);
    return res
}

fn in_order_rec_helper(root: &Option<Box<TreeNode>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        in_order_rec_helper(&node.left, res);
        res.push(node.val);
        in_order_rec_helper(&node.right, res);
    }
}

// O(n) time | O(h) space
pub fn post_order_rec(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];
    post_order_rec_helper(root, &mut res);
    return res;
}

fn post_order_rec_helper(root: &Option<Box<TreeNode>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        post_order_rec_helper(&node.left, res);
        post_order_rec_helper(&node.right, res);
        res.push(node.val);
    }
}

#[test]
pub fn test_dfs_rec() {

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


    assert_eq!(pre_order_rec(&root), vec![5, 2, 1, 3, 8, 7, 9]);
    assert_eq!(in_order_rec(&root), vec![1, 2, 3, 5, 7, 8, 9]);
    assert_eq!(post_order_rec(&root), vec![1, 3, 2, 7, 9, 8, 5]);
}