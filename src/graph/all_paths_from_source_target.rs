use std::collections::VecDeque;

/**
 * https://leetcode.com/problems/all-paths-from-source-to-target/
 *
 * Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1,
 * find all possible paths from node 0 to node n - 1 and return them in any order.
 *
 * The graph is given as follows: graph[i] is a list of all nodes you can visit from node i
 * (i.e., there is a directed edge from node i to node graph[i][j]).
 */

// O(V * 2 ^ V) time | O(V) space
pub fn all_paths_source_target_rec(graph: &[&[usize]]) -> Vec<Vec<usize>> {
    if graph.is_empty() {
        return vec![];
    }

    let mut res = vec![];
    let start = 0;
    let mut ans = vec![start];
    dfs(graph, start, &mut ans, &mut res);
    res
}

fn dfs(graph: &[&[usize]], v: usize, ans: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
    if graph.len() - 1 == v {
        res.push(ans.clone());
        return;
    }

    for &u in graph[v] {
        ans.push(u);
        dfs(graph, u, ans, res);
        ans.pop();
    }
}

// O(2 ^ V * V ^ 2) time | O(2 ^ V) space
pub fn all_paths_source_target(graph: &[&[usize]]) -> Vec<Vec<usize>> {
    if graph.is_empty() {
        return vec![];
    }

    let mut res = vec![];

    let mut stack = VecDeque::new();
    let start: usize = 0;
    stack.push_back((start, vec![start]));

    while let Some((v, path)) = stack.pop_back() {

        if v == graph.len() - 1 {
            res.push(path.clone());
        }

        for &u in graph[v] {
            let mut npath = path.clone();
            npath.push(u);
            stack.push_back((u, npath));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::graph::all_paths_from_source_target::{all_paths_source_target_rec, all_paths_source_target};

    const GRAPH: &[&[usize]] = &[
        &[1, 2],
        &[3],
        &[3],
        &[],
    ];

    const GRAPH1: &[&[usize]] = &[
        &[4, 3, 1],
        &[3, 2, 4],
        &[3],
        &[4],
        &[],
    ];

    #[test]
    fn test_all_paths_from_source_target_rec() {
        assert_eq!(all_paths_source_target_rec(GRAPH), vec![vec![0, 1, 3], vec![0, 2, 3]]);
    }

    #[test]
    fn test_all_paths_from_source_target_rec_case1() {
        assert_eq!(all_paths_source_target_rec(GRAPH1), vec![vec![0, 4], vec![0, 3, 4], vec![0, 1, 3, 4], vec![0, 1, 2, 3, 4], vec![0, 1, 4]]);
    }

    #[test]
    fn test_all_paths_from_source_target() {
        assert_eq!(all_paths_source_target(GRAPH), vec![vec![0, 2, 3], vec![0, 1, 3]]);
    }

    #[test]
    fn test_all_paths_from_source_target_case1() {
        assert_eq!(all_paths_source_target(GRAPH1), vec![vec![0, 1, 4], vec![0, 1, 2, 3, 4], vec![0, 1, 3, 4], vec![0, 3, 4], vec![0, 4]]);
    }
}