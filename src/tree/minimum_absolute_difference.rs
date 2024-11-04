use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn get_minimum_difference(root: &Option<Box<TreeNode>>) -> i32 {

    let mut min_diff = i32::MAX;

    if let Some(node) = root {
        let mut stack = vec![];
        let mut curr = Some(node);


        let mut prev = i32::MAX;

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(node);
                curr = node.left.as_ref();
            }

            if let Some(node) = stack.pop() {
                min_diff = (node.val - prev).abs().min(min_diff);

                prev = node.val;
                curr = node.right.as_ref();
            }
        }
    }

    min_diff
}

#[cfg(test)]
mod tests {
    use crate::tree::minimum_absolute_difference::get_minimum_difference;
    use crate::tree::TreeNode;

    #[test]
    fn test_get_minimum_difference() {
        let root = Some(Box::new(TreeNode::new(
            4,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::leaf(6))),
        )));

        assert_eq!(get_minimum_difference(&root), 1);
    }

    #[test]
    fn test_minimum_absolute_difference_case1() {
        let root = Some(Box::new(TreeNode::new(
            5,
            Some(Box::new(TreeNode::leaf(0))),
            Some(Box::new(TreeNode::new(
                48,
                Some(Box::new(TreeNode::leaf(12))),
                Some(Box::new(TreeNode::leaf(50))),
            ))),
        )));

        assert_eq!(get_minimum_difference(&root), 2);
    }
}