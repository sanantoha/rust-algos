use std::rc::Rc;
use crate::graph::{DirectedEdge, EdgeWeightedDigraph};

// O(E * V) time | O(V) space
pub fn shortest_path(graph: EdgeWeightedDigraph, start: usize) -> (Vec<f64>, Vec<Option<usize>>) {
    let mut shortest = vec![f64::MAX; graph.v];
    shortest[start] = 0f64;
    let mut prev = vec![None; graph.v];

    for _ in 0..(graph.v - 1) {
        for edge in graph.edges() {
            relax(&mut shortest, &mut prev, edge);
        }
    }

    (shortest, prev)
}

fn relax(shortest: &mut Vec<f64>, prev: &mut Vec<Option<usize>>, edge: Rc<DirectedEdge>) {
    let new_weight = edge.weight() + shortest[edge.from()];
    if new_weight < shortest[edge.to()] {
        shortest[edge.to()] = new_weight;
        prev[edge.to()] = Some(edge.from());
    }
}

// O(E + V) time | O(V) space
pub fn find_negative_weight_circle(graph: EdgeWeightedDigraph, shortest: Vec<f64>, prev: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut v = None;

    for edge in graph.edges() {
        let new_weight = shortest[edge.from()] + edge.weight();
        if new_weight < shortest[edge.to()] {
            v = Some(edge.from());
        }
    }

    if v.is_none() {
        return vec![];
    }

    let mut res = vec![];
    res.push(v);
    let mut u = v.and_then(|x| prev[x]);

    while let Some(i) = u.take() {
        if v == Some(i) {
            break;
        }
        res.push(Some(i));
        u = prev[i];
    }

    res
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