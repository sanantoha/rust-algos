use crate::tree::TreeNode;

pub fn brunch_sums(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    vec![]
}

pub fn brunch_sums_iter(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::{brunch_sums, brunch_sums_iter};
    use crate::tree::TreeNode;

    #[test]
    fn test_brunch_sums() {
        let root = create_tree();

        let mut res = brunch_sums(&root);
        println!("{:?}", res);
        res.sort();
        assert_eq!(res, vec![10, 11, 15, 16, 18]);
    }

    #[test]
    fn test_brunch_sums_iter() {
        let root = create_tree();

        let mut res = brunch_sums_iter(&root);
        println!("{:?}", res);
        res.sort();
        assert_eq!(res, vec![10, 11, 15, 16, 18]);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(1,
                                               Some(Box::new(TreeNode::new(2,
                                                                           Some(Box::new(TreeNode::new(4,
                                                                                                       Some(Box::new(TreeNode::leaf(8))),
                                                                                                       Some(Box::new(TreeNode::leaf(9)))
                                                                           ))),
                                                                           Some(Box::new(TreeNode::new(5,
                                                                                                       Some(Box::new(TreeNode::leaf(10))),
                                                                                                       None
                                                                           )))
                                               ))),
                                               Some(Box::new(TreeNode::new(3,
                                                                           Some(Box::new(TreeNode::leaf(6))),
                                                                           Some(Box::new(TreeNode::leaf(7)))
                                               )))
        )));
        root
    }
}