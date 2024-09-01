use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn find_tilt(root: &Option<Box<TreeNode>>) -> i32 {
    helper(root).tilt
}

fn helper(root: &Option<Box<TreeNode>>) -> Info {
    if let Some(node) = root {
        let Info { sum: lsum, tilt: ltilt } = helper(&node.left);
        let Info { sum: rsum, tilt: rtilt } = helper(&node.right);

        let sum = lsum + rsum + node.val;
        let tilt = (lsum - rsum).abs() + ltilt + rtilt;

        return Info { sum, tilt };
    }

    Info { sum: 0, tilt: 0 }
}

struct Info {
    sum: i32,
    tilt: i32
}

#[cfg(test)]
mod tests {
    use crate::tree::{binary_tree_tilt::find_tilt, TreeNode};


    #[test]
    fn it_find_tilt() {
        let root = Some(Box::new(TreeNode::new(1, Some(Box::new(TreeNode::leaf(2))), Some(Box::new(TreeNode::leaf(3))))));

        assert_eq!(find_tilt(&root), 1);
    }

    #[test]
    fn it_find_tilt1() {
        let root = Some(Box::new(TreeNode::new(
            4,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(3))),
                Some(Box::new(TreeNode::leaf(5))),
            ))),
            Some(Box::new(TreeNode::new(
                9,
                None,
                Some(Box::new(TreeNode::leaf(7))),
            ))),
        )));

        assert_eq!(find_tilt(&root), 15);
    }

    #[test]
    fn it_find_titl2() {
        let root = Some(Box::new(TreeNode::new(
            21,
            Some(Box::new(TreeNode::new(
                7,
                Some(Box::new(TreeNode::new(
                    1,
                    Some(Box::new(TreeNode::leaf(3))),
                    Some(Box::new(TreeNode::leaf(3))),
                ))),
                Some(Box::new(TreeNode::leaf(1))),
            ))),
            Some(Box::new(TreeNode::new(
                14,
                Some(Box::new(TreeNode::leaf(2))),
                Some(Box::new(TreeNode::leaf(2))),
            ))),
        )));

        assert_eq!(find_tilt(&root), 9);
    }
}