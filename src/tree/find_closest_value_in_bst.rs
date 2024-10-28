use crate::tree::TreeNode;

// O(h) time | O(h) space
pub fn find_closest_value_in_bst_rec(root: &Option<Box<TreeNode>>, target: i32) -> Option<i32> {
    if let Some(node) = root {
        return helper(root, node.val, target);
    }
    None
}

fn helper(root: &Option<Box<TreeNode>>, mut closest: i32, target: i32) -> Option<i32> {
    if let Some(node) = root {

        if (target - node.val).abs() < (target - closest).abs() {
            closest = node.val;
        }

        if node.val > target {
            return helper(&node.left, closest, target);
        } else if node.val < target {
            return helper(&node.right, closest, target);
        } else {
            return Some(node.val);
        }
    }
    Some(closest)
}

// O(h) time | O(h) space
pub fn find_closest_value_in_bst(root: &Option<Box<TreeNode>>, target: i32) -> Option<i32> {

    let mut closest = root.as_ref().map(|x| x.val).unwrap_or(i32::MIN);

    let mut curr = root;

    while let Some(node) = curr {
        if (target - node.val).abs() < (target - closest).abs() {
            closest = node.val;
        }

        if node.val > target {
            curr = &node.left;
        } else if node.val < target {
            curr = &node.right;
        } else {
            return Some(node.val);
        }
    }

    Some(closest)
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::{find_closest_value_in_bst_rec, find_closest_value_in_bst};

    #[test]
    fn test_find_closest_value_in_bst_rec() {
        let root = create_tree();

        assert_eq!(find_closest_value_in_bst_rec(&root, 12), Some(13));
    }

    #[test]
    fn test_find_closest_value_in_bst() {
        let root = create_tree();

        assert_eq!(find_closest_value_in_bst(&root, 12), Some(13));
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            10,
            Some(Box::new(TreeNode::new(
                5,
                Some(Box::new(TreeNode::new(
                    2,
                    Some(Box::new(TreeNode::leaf(1))),
                    None,
                ))),
                Some(Box::new(TreeNode::leaf(5))),
            ))),
            Some(Box::new(TreeNode::new(
                15,
                Some(Box::new(TreeNode::new(
                    13,
                    None,
                    Some(Box::new(TreeNode::leaf(14))),
                ))),
                Some(Box::new(TreeNode::leaf(22))),
            ))),
        );

        Some(Box::new(root))
    }
}