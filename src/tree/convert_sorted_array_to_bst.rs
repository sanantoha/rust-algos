use crate::tree::TreeNode;

// O(n) time | O(log(n)) space
pub fn sorted_array_to_bst(arr: &[i32]) -> Option<Box<TreeNode>> {
    helper(arr, 0, arr.len() - 1)
}

fn helper(arr: &[i32], l: usize, r: usize) -> Option<Box<TreeNode>> {
    if l > r {
        return None;
    }

    let mid = l + (r - l) / 2;
    let val = arr[mid];
    let mut root = TreeNode::leaf(val);
    if let Some(mid_minus_one) = mid.checked_sub(1) {
        root.left = helper(arr, l, mid_minus_one);
    }
    root.right = helper(arr, mid + 1, r);

    Some(Box::new(root))
}

#[cfg(test)]
mod tests {
    use crate::tree::convert_sorted_array_to_bst::sorted_array_to_bst;
    use crate::tree::TreeNode;

    #[test]
    fn it_sorted_array_to_bst() {
        let arr: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let tree = sorted_array_to_bst(arr);
        println!("{:?}", tree);

        let exp_tree = Some(Box::new(TreeNode::new(5,
            Some(Box::new(TreeNode::new(2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::new(3,
                    None,
                    Some(Box::new(TreeNode::leaf(4)))
                )))
            ))),
            Some(Box::new(TreeNode::new(8,
                Some(Box::new(TreeNode::new(6,
                    None,
                    Some(Box::new(TreeNode::leaf(7)))
                ))),
                Some(Box::new(TreeNode::new(9,
                    None,
                    Some(Box::new(TreeNode::leaf(10)))
                ))),
            ))),
        )));

        assert_eq!(tree, exp_tree);
    }
}