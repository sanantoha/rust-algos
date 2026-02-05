use crate::tree::TreeNode;

pub fn find_node_distance_k(root: &Option<Box<TreeNode>>, target: i32, k: i32) -> Vec<i32> {
    vec![]
}

pub fn find_node_distance_k_rec(root: &Option<Box<TreeNode>>, target: i32, k: i32) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_node_distance_k() {
        let root = create_tree();

        let mut res = find_node_distance_k(&root, 3, 2);
        res.sort();
        assert_eq!(res, vec![2, 7, 8]);
    }

    #[test]
    fn test_find_node_distance_k_rec() {
        let root = create_tree();

        let mut res = find_node_distance_k_rec(&root, 3, 2);
        res.sort();
        assert_eq!(res, vec![2, 7, 8]);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(1,
                                               Some(Box::new(TreeNode::new(2,
                                                                           Some(Box::new(TreeNode::leaf(4))),
                                                                           Some(Box::new(TreeNode::leaf(5)))
                                               ))),
                                               Some(Box::new(TreeNode::new(3,
                                                                           None,
                                                                           Some(Box::new(TreeNode::new(6,
                                                                                                       Some(Box::new(TreeNode::leaf(7))),
                                                                                                       Some(Box::new(TreeNode::leaf(8)))
                                                                           )))
                                               )))
        )));
        root
    }
}