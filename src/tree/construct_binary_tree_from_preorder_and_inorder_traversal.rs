use std::collections::HashMap;
use crate::tree::TreeNode;

// O(n) time | O(n) space
pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Box<TreeNode>> {

    let mut inorder_map = HashMap::new();
    for (i, v) in inorder.iter().enumerate() {
        inorder_map.insert(*v, i);
    }

    let mut index = Index { idx: 0 };

    helper(preorder, &inorder_map, &mut index, 0, preorder.len() - 1)
}

fn helper(preorder: &[i32], inorder_map: &HashMap<i32, usize>, index: &mut Index, l: usize, r: usize) -> Option<Box<TreeNode>> {
    if l > r {
        return None;
    }

    let v = preorder[index.idx];
    index.idx += 1;

    let mut root = TreeNode::leaf(v);
    if let Some(&mid) = inorder_map.get(&v) {
        if let Some(mid_minus_one) = mid.checked_sub(1) {
            root.left = helper(preorder, inorder_map, index, l, mid_minus_one);
        }
        root.right = helper(preorder, inorder_map, index, mid + 1, r);
    }

    Some(Box::new(root))
}

pub struct Index {
    pub idx: usize,
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::build_tree;

    #[test]
    fn test_build_tree() {
        let preorder: &[i32] = &[3, 9, 20, 15, 7];
        let inorder: &[i32] = &[9, 3, 15, 20, 7];

        let tree = build_tree(preorder, inorder);

        /*
                     3
                  /    \
                9      20
                     /   \
                   15     7
         */

        let exp_tree = Some(
            Box::new(TreeNode::new(3,
                Some(Box::new(TreeNode::leaf(9))),
                Some(Box::new(TreeNode::new(20,
                    Some(Box::new(TreeNode::leaf(15))),
                    Some(Box::new(TreeNode::leaf(7))),
                ))),
            ))
        );

        assert_eq!(tree, exp_tree);
    }
}