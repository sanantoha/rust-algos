use crate::tree::TreeNode;

// O(n) time | O(1) space
pub fn find_mode(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    if let Some(node) = root {

        let mut stack = vec![];
        let mut curr_opt = Some(node);

        let mut res = vec![];

        let mut prev = i32::MIN;

        let mut counter = 0;
        let mut max_count = 0;

        while !stack.is_empty() || curr_opt.is_some() {
            while let Some(curr) = curr_opt.take() {
                stack.push(curr);
                curr_opt = curr.left.as_ref();
            }

            if let Some(curr) = stack.pop() {

                if prev == curr.val {
                    counter += 1;
                } else {
                    counter = 1;
                }

                if counter == max_count {
                    res.push(curr.val)
                } else if counter > max_count {
                    max_count = counter;
                    res = vec![curr.val];
                }

                prev = curr.val;
                curr_opt = curr.right.as_ref();
            }
        }

        return res;
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use super::find_mode;

    #[test]
    fn test_find_mode() {

        let root = Some(Box::new(TreeNode::new(1,
            None,
           Some(Box::new(TreeNode::new(2,
               Some(Box::new(TreeNode::leaf(2))),
               None
           )))
        )));

        assert_eq!(find_mode(&root), vec![2]);
    }

    #[test]
    fn test_find_mode_case1() {

        let root = Some(Box::new(TreeNode::leaf(0)));

        assert_eq!(find_mode(&root), vec![0]);
    }

    #[test]
    fn test_find_mode_case2() {

        let root = Some(Box::new(TreeNode::new(5,
                       Some(Box::new(TreeNode::new(3,
                           Some(Box::new(TreeNode::leaf(1))),
                           Some(Box::new(TreeNode::leaf(3)))
                       ))),
                       Some(Box::new(TreeNode::new(7,
                           Some(Box::new(TreeNode::leaf(5))),
                           Some(Box::new(TreeNode::leaf(7)))
                       )))
        )));

        assert_eq!(find_mode(&root), vec![3, 5, 7]);
    }
}
