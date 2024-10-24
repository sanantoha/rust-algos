use crate::tree::TreeNode;

// O(n) time | O(h) space
pub fn brunch_sums(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];
    backtrack(root, 0, &mut res);
    res
}

fn backtrack(root: &Option<Box<TreeNode>>, mut sum: i32, sums: &mut Vec<i32>) {
    if let Some(node) = root {

        sum += node.val;
        if node.left.is_none() && node.right.is_none() {
            sums.push(sum);
            return;
        }

        backtrack(&node.left, sum, sums);
        backtrack(&node.right, sum, sums);
    }
}

// O(n) time | O(h) space
pub fn brunch_sums_iter(root: &Option<Box<TreeNode>>) -> Vec<i32> {

    let mut res = vec![];

    let mut stack = vec![(root, 0)];

    while let Some((node, sum)) = stack.pop() {
        if let Some(curr) = node {
            let new_sum = sum + curr.val;

            if curr.left.is_none() && curr.right.is_none() {
                res.push(new_sum);
            }

            stack.push((&curr.left, new_sum));
            stack.push((&curr.right, new_sum));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::tree::brunch_sums::{brunch_sums, brunch_sums_iter};
    use crate::tree::TreeNode;

    #[test]
    fn test_brunch_sums() {
        let root = create_tree();

        let mut res = brunch_sums(&root);
        println!("{:?}", res);
        res.sort();
        assert_eq!(res, vec![10, 11, 15, 16, 18]);
    }

    #[test]
    fn test_brunch_sums_iter() {
        let root = create_tree();

        let mut res = brunch_sums_iter(&root);
        println!("{:?}", res);
        res.sort();
        assert_eq!(res, vec![10, 11, 15, 16, 18]);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(1,
                                               Some(Box::new(TreeNode::new(2,
                                                                           Some(Box::new(TreeNode::new(4,
                                                                                                       Some(Box::new(TreeNode::leaf(8))),
                                                                                                       Some(Box::new(TreeNode::leaf(9)))
                                                                           ))),
                                                                           Some(Box::new(TreeNode::new(5,
                                                                                                       Some(Box::new(TreeNode::leaf(10))),
                                                                                                       None
                                                                           )))
                                               ))),
                                               Some(Box::new(TreeNode::new(3,
                                                                           Some(Box::new(TreeNode::leaf(6))),
                                                                           Some(Box::new(TreeNode::leaf(7)))
                                               )))
        )));
        root
    }
}
