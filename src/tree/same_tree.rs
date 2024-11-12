use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn is_same_tree(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
    if let (Some(c1), Some(c2)) = (t1, t2) {
        c1.val == c2.val && is_same_tree(&c1.left, &c2.left) && is_same_tree(&c1.right, &c2.right)
    } else if t1.is_none() && t2.is_none() {
        true
    } else {
        false
    }
}

// O(n) time | O(h) space
pub fn is_same_tree_iter(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {

    if let (Some(c1), Some(c2)) = (t1, t2) {

        let mut stack = vec![c1, c2];

        while let Some(c1) = stack.pop() {
            if let Some(c2) = stack.pop() {

                if c1.val != c2.val {
                    return false;
                }

                if let (Some(l1), Some(l2)) = (c1.left.as_ref(), c2.left.as_ref()) {
                    stack.push(l1);
                    stack.push(l2);
                } else if !(c1.left.is_none() && c2.left.is_none()) {
                    return false;
                }

                if let (Some(r1), Some(r2)) = (c1.right.as_ref(), c2.right.as_ref()) {
                    stack.push(r1);
                    stack.push(r2);
                } else if !(c1.right.is_none() && c2.right.is_none()) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    } else if t1.is_none() && t2.is_none() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::{is_same_tree, is_same_tree_iter};

    #[test]
    fn test_is_same_tree() {
        let t1 = create_tree1();
        let t2 = create_tree1();
        assert!(is_same_tree(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_iter() {
        let t1 = create_tree1();
        let t2 = create_tree1();
        assert!(is_same_tree_iter(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_neg() {
        let t1 = create_tree1();
        let t2 = create_tree2();
        assert!(!is_same_tree(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_iter_neg() {
        let t1 = create_tree1();
        let t2 = create_tree2();
        assert!(!is_same_tree_iter(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_case1() {
        let t1 = create_tree3();
        let t2 = create_tree31();

        assert!(!is_same_tree(&t1, &t2));
    }

    #[test]
    fn test_is_same_tree_iter_case1() {
        let t1 = create_tree3();
        let t2 = create_tree31();
        assert!(!is_same_tree_iter(&t1, &t2));
    }

    fn create_tree1() -> Option<Box<TreeNode>> {
        let tree = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::leaf(15))),
            ))),
        );

        Some(Box::new(tree))
    }

    fn create_tree2() -> Option<Box<TreeNode>> {
        let tree = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                Some(Box::new(TreeNode::leaf(444444444))),
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::leaf(15))),
            ))),
        );

        Some(Box::new(tree))
    }

    fn create_tree3() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            Some(Box::new(TreeNode::new(
                2,
                None,
                Some(Box::new(TreeNode::leaf(3))),
            ))),
            Some(Box::new(TreeNode::new(
                2,
                None,
                Some(Box::new(TreeNode::leaf(3))),
            ))),
        );
        Some(Box::new(root))
    }

    fn create_tree31() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(3))),
                None,
            ))),
            Some(Box::new(TreeNode::new(
                2,
                None,
                Some(Box::new(TreeNode::leaf(3))),
            ))),
        );
        Some(Box::new(root))
    }
}