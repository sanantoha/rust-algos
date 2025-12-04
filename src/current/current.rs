use crate::tree::TreeNode;

pub fn max_depth_rec(root: &Option<Box<TreeNode>>) -> i32 {
    0
}

pub fn max_depth_bfs(root: &Option<Box<TreeNode>>) -> i32 {
    0
}

pub fn max_depth_dfs_iter(root: &Option<Box<TreeNode>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{max_depth_bfs, max_depth_dfs_iter, max_depth_rec};
    use crate::tree::TreeNode;

    #[test]
    fn test_max_depth_rec() {
        let root = &create_tree();

        assert_eq!(max_depth_rec(root), 4);
    }

    #[test]
    fn test_max_depth_rec_case1() {
        let root = &create_tree1();

        assert_eq!(max_depth_rec(root), 3);
    }

    #[test]
    fn test_max_depth_rec_case2() {
        let root = &create_tree2();

        assert_eq!(max_depth_rec(root), 2);
    }

    #[test]
    fn test_max_depth_bfs() {
        let root = &create_tree();

        assert_eq!(max_depth_bfs(root), 4);
    }

    #[test]
    fn test_max_depth_bfs_case1() {
        let root = &create_tree1();

        assert_eq!(max_depth_bfs(root), 3);
    }

    #[test]
    fn test_max_depth_bfs_case2() {
        let root = &create_tree2();

        assert_eq!(max_depth_bfs(root), 2);
    }

    #[test]
    fn test_max_depth_dfs_iter() {
        let root = &create_tree();

        assert_eq!(max_depth_dfs_iter(root), 4);
    }

    #[test]
    fn test_max_depth_dfs_iter_case1() {
        let root = &create_tree1();

        assert_eq!(max_depth_dfs_iter(root), 3);
    }

    #[test]
    fn test_max_depth_dfs_iter_case2() {
        let root = &create_tree2();

        assert_eq!(max_depth_dfs_iter(root), 2);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                None,
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::new(
                    20,
                    None,
                    Some(Box::new(TreeNode::leaf(25))),
                ))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree1() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            3,
            Some(Box::new(TreeNode::leaf(9))),
            Some(Box::new(TreeNode::new(
                20,
                Some(Box::new(TreeNode::leaf(15))),
                Some(Box::new(TreeNode::leaf(7))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree2() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            None,
            Some(Box::new(TreeNode::leaf(2))),
        );

        Some(Box::new(root))
    }
}