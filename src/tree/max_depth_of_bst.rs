use std::collections::VecDeque;
use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn max_depth_rec(root: &Option<Box<TreeNode>>) -> i32 {
    helper(root, 0)
}

fn helper(root: &Option<Box<TreeNode>>, depth: i32) -> i32 {
    if let Some(node) = root {
        helper(&node.left, depth + 1).max(helper(&node.right, depth + 1))
    } else {
        depth
    }
}

// O(n) time | O(n) space
pub fn max_depth_bfs(root: &Option<Box<TreeNode>>) -> i32 {
    let mut depth = 0;
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);



        while !queue.is_empty() {
            let mut size = queue.len();

            depth += 1;

            while size > 0 {
                size -= 1;
                if let Some(curr) = queue.pop_front() {

                    if let Some(left) = &curr.left {
                        queue.push_back(left);
                    }

                    if let Some(right) = &curr.right {
                        queue.push_back(right);
                    }

                }
            }
        }
    }

    depth
}

// O(n) time | O(h) space
pub fn max_depth_dfs_iter(root: &Option<Box<TreeNode>>) -> i32 {

    let mut max_depth = 0;

    if let Some(node) = root {
        let mut stack = vec![(node, 0)];

        while let Some((node, depth)) = stack.pop() {

            max_depth = max_depth.max(depth);

            if let Some(left) = &node.left {
                stack.push((left, depth + 1));
            }
            if let Some(right) = &node.right {
                stack.push((right, depth + 1));
            }
        }
    }

    max_depth + 1
}

#[cfg(test)]
mod tests {
    use crate::tree::max_depth_of_bst::{max_depth_bfs, max_depth_dfs_iter, max_depth_rec};
    use crate::tree::TreeNode;

    #[test]
    fn test_max_depth_rec() {
        let root = &create_tree();

        assert_eq!(max_depth_rec(root), 4);
    }

    #[test]
    fn test_max_depth_rec_case1() {
        let root = &create_tree1();

        assert_eq!(max_depth_rec(root), 3);
    }

    #[test]
    fn test_max_depth_rec_case2() {
        let root = &create_tree2();

        assert_eq!(max_depth_rec(root), 2);
    }

    #[test]
    fn test_max_depth_bfs() {
        let root = &create_tree();

        assert_eq!(max_depth_bfs(root), 4);
    }

    #[test]
    fn test_max_depth_bfs_case1() {
        let root = &create_tree1();

        assert_eq!(max_depth_bfs(root), 3);
    }

    #[test]
    fn test_max_depth_bfs_case2() {
        let root = &create_tree2();

        assert_eq!(max_depth_bfs(root), 2);
    }

    #[test]
    fn test_max_depth_dfs_iter() {
        let root = &create_tree();

        assert_eq!(max_depth_dfs_iter(root), 4);
    }

    #[test]
    fn test_max_depth_dfs_iter_case1() {
        let root = &create_tree1();

        assert_eq!(max_depth_dfs_iter(root), 3);
    }

    #[test]
    fn test_max_depth_dfs_iter_case2() {
        let root = &create_tree2();

        assert_eq!(max_depth_dfs_iter(root), 2);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            5,
            Some(Box::new(TreeNode::new(
                2,
                Some(Box::new(TreeNode::leaf(1))),
                None,
            ))),
            Some(Box::new(TreeNode::new(
                10,
                Some(Box::new(TreeNode::leaf(7))),
                Some(Box::new(TreeNode::new(
                    20,
                    None,
                    Some(Box::new(TreeNode::leaf(25))),
                ))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree1() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            3,
            Some(Box::new(TreeNode::leaf(9))),
            Some(Box::new(TreeNode::new(
                20,
                Some(Box::new(TreeNode::leaf(15))),
                Some(Box::new(TreeNode::leaf(7))),
            ))),
        );

        Some(Box::new(root))
    }

    fn create_tree2() -> Option<Box<TreeNode>> {
        let root = TreeNode::new(
            1,
            None,
            Some(Box::new(TreeNode::leaf(2))),
        );

        Some(Box::new(root))
    }
}