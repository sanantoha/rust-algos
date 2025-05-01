use crate::graph::EdgeWeightedDigraph;

pub fn dfs_rec(graph: &EdgeWeightedDigraph, start: usize) -> Vec<usize> {
    vec![]
}

pub fn dfs(graph: &EdgeWeightedDigraph, start: usize) -> Vec<usize> {
    vec![]
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::{dfs_rec, dfs};
    use crate::graph::EdgeWeightedDigraph;

    const PATH: &str = "src/graph/dfs.txt";

    #[test]
    fn it_dfs_rec() {
        if let Ok(graph) = EdgeWeightedDigraph::from_file(PathBuf::from(PATH)) {
            println!("{}", graph);

            let res = dfs_rec(&graph, 0);
            println!("{:?}", res);
            assert_eq!(res, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
        }
    }

    #[test]
    fn it_dfs() {
        if let Ok(graph) = EdgeWeightedDigraph::from_file(PathBuf::from(PATH)) {
            println!("{}", graph);

            let res = dfs(&graph, 0);
            println!("{:?}", res);
            assert_eq!(res, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14])
        }
    }
}