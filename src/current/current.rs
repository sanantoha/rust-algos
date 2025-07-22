use crate::graph::EdgeWeightedDigraph;

pub fn bfs(graph: &EdgeWeightedDigraph, start: usize) -> Vec<usize> {
    vec![]
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::bfs;
    use crate::graph::EdgeWeightedDigraph;


    #[test]
    fn it_bfs() {
        if let Ok(graph) = EdgeWeightedDigraph::from_file(PathBuf::from("src/graph/bfs.txt")) {
            println!("{}", graph);

            let res = bfs(&graph, 0);
            println!("{:?}", res);
            assert_eq!(res, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]);
        }
    }
}