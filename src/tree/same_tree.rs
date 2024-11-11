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

                if let Some(l) = c1.left.as_ref() {
                    stack.push(l);
                }
                if let Some(l) = c2.left.as_ref() {
                    stack.push(l);
                }

                if let Some(r) = c1.right.as_ref() {
                    stack.push(r);
                }
                if let Some(r) = c2.right.as_ref() {
                    stack.push(r);
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
}