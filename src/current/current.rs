/**
 * https://leetcode.com/problems/all-paths-from-source-to-target/
 *
 * Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1,
 * find all possible paths from node 0 to node n - 1 and return them in any order.
 *
 * The graph is given as follows: graph[i] is a list of all nodes you can visit from node i
 * (i.e., there is a directed edge from node i to node graph[i][j]).
 */

pub fn all_paths_source_target_rec(graph: &[&[usize]]) -> Vec<Vec<usize>> {
    vec![]
}


pub fn all_paths_source_target(graph: &[&[usize]]) -> Vec<Vec<usize>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::{all_paths_source_target, all_paths_source_target_rec};

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