
#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn single(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }

    pub fn new(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode {
            val: val,
            left: left,
            right: right,
        }
    }
}