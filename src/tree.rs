pub mod balanced_binary_tree;
pub mod dfs_tree_traverse_rec;
pub mod dfs_tree_traverse;
pub mod bfs_tree_traverse;


#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn single(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}