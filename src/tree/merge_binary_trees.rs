use crate::tree::TreeNode;

// O(n) time | O(h) space - where n, h for the smallest tree
pub fn merge_binary_trees(left: &mut Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if left.is_none() {
        return right.clone();
    }
    if right.is_none() {
        return left.take();
    }

    if let (Some(l), Some(r)) = (left, right) {
        l.val += r.val;

        l.left = merge_binary_trees(&mut l.left, &r.left);
        l.right = merge_binary_trees(&mut l.right, &r.right);

        return Some(std::mem::take(l));
    }
    None
}

// O(n) time | O(h) space - where n, h for the smallest tree
pub fn merge_binary_trees_iter(left: &mut Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if left.is_none() {
        return right.clone();
    }
    if right.is_none() {
        return left.take();
    }

    let mut stack1 = vec![left.as_mut()];
    let mut stack2 = vec![right.as_ref()];

    while let Some(Some(curr1)) = stack1.pop() {
        if let Some(Some(curr2)) = stack2.pop() {

            curr1.val += curr2.val;

            if curr1.left.is_none() {
                curr1.left = curr2.left.clone();
            } else {
                if let Some(l) = curr1.left.as_mut() {
                    stack1.push(Some(l));
                }
                stack2.push(curr2.left.as_ref());
            }

            if curr1.right.is_none() {
                curr1.right = curr2.right.clone();
            } else {
                if let Some(r) = curr1.right.as_mut() {
                    stack1.push(Some(r));
                }
                stack2.push(curr2.right.as_ref());
            }
        }
    }

    left.take()
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use crate::tree::TreeNode;
    use super::{merge_binary_trees, merge_binary_trees_iter};

    #[test]
    fn test_merge_binary_trees() {
        let mut left = create_left_tree();
        let right = create_right_tree();
        let exp_tree = create_exp_tree();

        assert_equal(merge_binary_trees(&mut left, &right), exp_tree);
    }

    #[test]
    fn test_merge_binary_trees_iter() {
        let mut left = create_left_tree();
        let right = create_right_tree();
        let exp_tree = create_exp_tree();

        assert_equal(merge_binary_trees_iter(&mut left, &right), exp_tree);
    }

    fn create_left_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(1,
            Some(Box::new(TreeNode::new(3,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::leaf(4)))
            ))),
            Some(Box::new(TreeNode::leaf(2)))
        );

        Some(Box::new(root))
    }

    fn create_right_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(1,
             Some(Box::new(TreeNode::new(5,
                 Some(Box::new(TreeNode::leaf(2))),
                 None
             ))),
             Some(Box::new(TreeNode::new(9,
                 Some(Box::new(TreeNode::leaf(7))),
                 Some(Box::new(TreeNode::leaf(6)))
             )))
        );

        Some(Box::new(root))
    }

    fn create_exp_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(2,
             Some(Box::new(TreeNode::new(8,
                  Some(Box::new(TreeNode::leaf(9))),
                  Some(Box::new(TreeNode::leaf(4)))
             ))),
             Some(Box::new(TreeNode::new(11,
                  Some(Box::new(TreeNode::leaf(7))),
                  Some(Box::new(TreeNode::leaf(6)))
             )))
        );

        Some(Box::new(root))
    }
}

