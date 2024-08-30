use std::collections::LinkedList;

use super::TreeNode;

// O(n) time | O(n) space
pub fn level_order(root: Box<TreeNode>) -> Vec<Vec<i32>> {

    let mut res = vec![];

    let mut queue = LinkedList::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let mut size = queue.len();

        let mut sub_res = vec![];

        while size > 0 {
            size -= 1;

            if let Some(node) = queue.pop_front() {
                sub_res.push(node.val);

                if let Some(left) = node.left {
                    queue.push_back(left);
                }
                if let Some(right) = node.right {
                    queue.push_back(right);
                }
            }
        }

        res.push(sub_res);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::level_order;
    use super::TreeNode;


    #[test]
    fn it_level_order() {
        let root = Box::new(TreeNode::new(5,
            Some(Box::new(TreeNode::new(2,
                Some(Box::new(TreeNode::single(1))),
                Some(Box::new(TreeNode::single(3)))))),
            Some(Box::new(TreeNode::new(10,
                Some(Box::new(TreeNode::new(7,
                    Some(Box::new(TreeNode::single(6))),
                    Some(Box::new(TreeNode::single(8)))))),
                Some(Box::new(TreeNode::new(15,
                    Some(Box::new(TreeNode::single(14))),
                    Some(Box::new(TreeNode::single(17)))))))))),
        );
        
        let res = level_order(root);
        println!("{res:?}");
        assert_eq!(res, vec![vec![5], vec![2, 10], vec![1, 3, 7, 15], vec![6, 8, 14, 17]]);
    }
}