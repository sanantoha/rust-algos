use crate::graph::EdgeWeightedDigraph;

pub fn shortest_path(graph: EdgeWeightedDigraph, start: usize) -> (Vec<f64>, Vec<Option<usize>>) {
    (vec![], vec![])
}

pub fn find_negative_weight_circle(graph: EdgeWeightedDigraph, shortest: Vec<f64>, prev: Vec<Option<usize>>) -> Vec<Option<usize>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::path::PathBuf;
    use crate::graph::EdgeWeightedDigraph;
    use super::{find_negative_weight_circle, shortest_path};

    #[test]
    fn test_shortest_path() {
        if let Ok(graph) = get_graph() {
            println!("{}", graph);

            let res = shortest_path(graph, 0);
            println!("{:?}", res);
            assert_eq!(res, (vec![-9.0, -20.0, -18.0, -2.0, -11.0], vec![Some(4), Some(2), Some(4), Some(0), Some(1)]));
        }
    }

    #[test]
    fn test_find_negative_weight_circle() {
        if let Ok(graph) = get_graph() {
            println!("{}", graph);

            let shortest = vec![-9.0, -20.0, -18.0, -2.0, -11.0];
            let prev = vec![Some(4), Some(2), Some(4), Some(0), Some(1)];

            let res = find_negative_weight_circle(graph, shortest, prev);
            println!("{:?}", res);
            assert_eq!(res, vec![Some(1), Some(2), Some(4)]);
        }
    }

    fn get_graph() -> Result<EdgeWeightedDigraph, Error> {
        EdgeWeightedDigraph::from_file(PathBuf::from("src/graph/bellmanFord.txt"))
    }
}