use std::collections::LinkedList;

use super::TreeNode;


// O(n) time | O(n) space
pub fn left_view(root: Box<TreeNode>) -> Vec<i32> {

    let mut queue = LinkedList::new();
    queue.push_back(root);

    let mut res = vec![];

    while !queue.is_empty() {
        let mut size = queue.len();

        let mut is_first_node = true;

        while size > 0 {
            size -= 1;

            if let Some(node) = queue.pop_front() {

                if is_first_node {
                    res.push(node.val);
                    is_first_node = false;
                }

                if let Some(left) = node.left {
                    queue.push_back(left);
                }

                if let Some(right) = node.right {
                    queue.push_back(right);
                }                
            }
        }
    }
    
    res
}

#[cfg(test)]
mod tests {

    use super::TreeNode;
    use super::left_view;

    #[test]
    fn it_left_view() {
        let root = Box::new(TreeNode::new(1,
            Some(Box::new(TreeNode::leaf(2))),
            Some(Box::new(TreeNode::new(3,
                Some(Box::new(TreeNode::new(4,
                    Some(Box::new(TreeNode::leaf(5))),
                    Some(Box::new(TreeNode::new(6,
                        None,
                        Some(Box::new(TreeNode::leaf(7))))))))),
                Some(Box::new(TreeNode::leaf(8)))))),
            ));

        let res = left_view(root);
        println!("{res:?}");
        assert_eq!(res, vec![1, 2, 4, 5, 7]);
    }
}