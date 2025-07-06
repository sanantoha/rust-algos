use crate::graph::EdgeWeightedDigraph;

pub fn shortest_path(graph: &EdgeWeightedDigraph, start: usize) -> (Vec<f64>, Vec<Option<usize>>) {
    (vec![], vec![])
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::graph::EdgeWeightedDigraph;
    use super::shortest_path;

    // ShortestPath{shortest=[0.0, 5.0, 8.0, 4.0, 7.0], prev=[-1, 3, 1, 0, 3]}
    #[test]
    fn test_shortest_path() {
        if let Ok(graph) = EdgeWeightedDigraph::from_file(PathBuf::from("src/graph/dijkstraShortestPath.txt")) {
            println!("{}", graph);

            let res = shortest_path(&graph, 0);
            println!("{:?}", res);

            assert_eq!(res, (vec![0.0, 5.0, 8.0, 4.0, 7.0], vec![None, Some(3), Some(1), Some(0), Some(3)]))
        }
    }
}