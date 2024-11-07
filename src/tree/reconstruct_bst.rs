use crate::tree::TreeNode;

// O(n ^ 2) time | O(n ^ 2) space
pub fn reconstruct_bst(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }

    let val = arr[0];
    let mut root = TreeNode::leaf(val);
    let lst = &arr[1..];
    let less: Vec<i32> = lst.to_vec().into_iter().filter(|x| *x < val).collect();
    let greater_or_equal: Vec<i32> = lst.to_vec().into_iter().filter(|x| *x >= val).collect();
    root.left = reconstruct_bst(less.as_slice());
    root.right = reconstruct_bst(greater_or_equal.as_slice());

    Some(Box::new(root))
}

// O(n) time | O(n) space
pub fn reconstruct_bst1(arr: &[i32]) -> Option<Box<TreeNode>> {
    if arr.is_empty() {
        return None;
    }
    let mut idx = 0;

    helper(arr, &mut idx, i32::MIN, i32::MAX)
}

fn helper(arr: &[i32], idx: &mut usize, min: i32, max: i32) -> Option<Box<TreeNode>> {
    if *idx == arr.len() {
        return None;
    }
    let val = arr[*idx];
    if val < min || val >= max {
        return None;
    }

    *idx += 1;
    let left = helper(arr, idx, min, val);
    let right = helper(arr, idx, val, max);
    Some(Box::new(TreeNode::new(val, left, right)))
}

#[cfg(test)]
mod tests {
    use crate::tree::reconstruct_bst::{reconstruct_bst, reconstruct_bst1};
    use crate::tree::TreeNode;

    const ARR: &[i32] = &[10, 4, 2, 1, 3, 17, 19, 18];

    #[test]
    fn test_reconstruct_bst() {
        let exp_tree = create_exp_tree();

        let res = reconstruct_bst(ARR);
        assert_eq!(res, exp_tree)
    }

    #[test]
    fn test_reconstruct_bst1() {
        let exp_tree = create_exp_tree();

        let res = reconstruct_bst1(ARR);
        assert_eq!(res, exp_tree)
    }

    fn create_exp_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(10,
            Some(Box::new(TreeNode::new(4,
                Some(Box::new(TreeNode::new(2,
                    Some(Box::new(TreeNode::leaf(1))),
                    Some(Box::new(TreeNode::leaf(3)))
                ))),
                None
            ))),
            Some(Box::new(TreeNode::new(17,
                None,
                Some(Box::new(TreeNode::new(19,
                    Some(Box::new(TreeNode::leaf(18))),
                    None
                )))
            ))),
        )));

        root
    }
}