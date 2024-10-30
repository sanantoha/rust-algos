use std::collections::{HashMap, HashSet, VecDeque};
use crate::tree::TreeNode;

// O(n) time | O(n) space
pub fn find_node_distance_k(root: &Option<Box<TreeNode>>, target: i32, k: i32) -> Vec<i32> {
    if root.is_none() || k < 0 {
        return vec![];
    }
    let mut parents = HashMap::new();
    enrich_parents(root, &None, &mut parents);

    let target_node_opt = find_target(root, &parents, target);

    if let Some(target_node) = target_node_opt {
        let mut queue = VecDeque::new();
        queue.push_back((target_node, 0));

        let mut seen = HashSet::new();
        seen.insert(target_node.val);

        let mut res = vec![];

        while let Some((node, distance)) = queue.pop_front() {
            if distance == k {
                res.push(node.val);
                while let Some((n, _)) = queue.pop_front() {
                    res.push(n.val);
                }
                break;
            }

            let neighbors = vec![node.left.as_ref(), node.right.as_ref(), parents.get(&node.val).map(|x| *x)];

            for neighbor_opt in neighbors {
                if let Some(neighbor) = neighbor_opt {
                    if seen.contains(&neighbor.val) {
                        continue;
                    }
                    queue.push_back((neighbor, distance + 1));
                    seen.insert(neighbor.val);
                }
            }
        }
        res
    } else {
        vec![]
    }
}

// O(n) time | O(n) space
pub fn find_node_distance_k_rec(root: &Option<Box<TreeNode>>, target: i32, k: i32) -> Vec<i32> {
    if root.is_none() || k < 0 {
        return vec![];
    }
    let mut parents = HashMap::new();
    enrich_parents(root, &None, &mut parents);

    let target_node = find_target(root, &parents, target);
    let mut res = vec![];
    let mut visited = HashSet::new();

    find_nodes(target_node.as_ref(), 0, k, &mut visited, &parents, &mut res);

    res
}

fn find_nodes(node_opt: Option<&Box<TreeNode>>, distance: i32, k: i32, visited: &mut HashSet<i32>, parents: &HashMap<i32, &Box<TreeNode>>, res: &mut Vec<i32>) {
    if let Some(node) = node_opt {
        if visited.contains(&node.val) {
            return;
        }
        visited.insert(node.val);
        if distance == k {
            res.push(node.val);
        } else {
            find_nodes(node.left.as_ref(), distance + 1, k, visited, parents, res);
            find_nodes(node.right.as_ref(), distance + 1, k, visited, parents, res);
            find_nodes(parents.get(&node.val).map(|x| *x), distance + 1, k, visited, parents, res);
        }
    }
}

fn find_target<'a>(root: &'a Option<Box<TreeNode>>, parents: &'a HashMap<i32, &'a Box<TreeNode>>, target: i32) -> &'a Option<Box<TreeNode>> {
    if let Some(r) = root {
        if r.val == target {
            return root
        }
    }
    if let Some(&parent) = parents.get(&target) {
        if let Some(l) = &parent.left {
            if l.val == target {
                return &parent.left
            }
        }
        return &parent.right;
    }

    &None
}

fn enrich_parents<'a>(root: &'a Option<Box<TreeNode>>, parent: &'a Option<Box<TreeNode>>, parents: &mut HashMap<i32, &'a Box<TreeNode>>) {
    if let Some(node) = root {
        if let Some(p) = parent {
            parents.insert(node.val, p);
        }

        enrich_parents(&node.left, root, parents);
        enrich_parents(&node.right, root, parents);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_node_distance_k() {
        let root = create_tree();

        let mut res = find_node_distance_k(&root, 3, 2);
        res.sort();
        assert_eq!(res, vec![2, 7, 8]);
    }

    #[test]
    fn test_find_node_distance_k_rec() {
        let root = create_tree();

        let mut res = find_node_distance_k_rec(&root, 3, 2);
        res.sort();
        assert_eq!(res, vec![2, 7, 8]);
    }

    fn create_tree() -> Option<Box<TreeNode>> {
        let root = Some(Box::new(TreeNode::new(1,
                       Some(Box::new(TreeNode::new(2,
                           Some(Box::new(TreeNode::leaf(4))),
                           Some(Box::new(TreeNode::leaf(5)))
                       ))),
                       Some(Box::new(TreeNode::new(3,
                           None,
                           Some(Box::new(TreeNode::new(6,
                               Some(Box::new(TreeNode::leaf(7))),
                               Some(Box::new(TreeNode::leaf(8)))
                           )))
                       )))
        )));
        root
    }
}