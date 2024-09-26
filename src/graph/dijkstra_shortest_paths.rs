use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;
use crate::graph::{DirectedEdge, EdgeWeightedDigraph};

// O((E + V) * log(V)) time | O(V) space
pub fn shortest_path(graph: &EdgeWeightedDigraph, start: usize) -> (Vec<f64>, Vec<Option<usize>>) {
    let mut shortest = vec![f64::MAX; graph.v];
    shortest[start] = 0f64;
    let mut path = vec![None; graph.v];

    let mut heap = BinaryHeap::new();
    let start_edge = Rc::new(DirectedEdge::new(start, start, 0f64));
    heap.push(Reverse(start_edge));

    while let Some(Reverse(min_edge)) = heap.pop() {
        for edge in graph.adj(min_edge.to()) {
            relax(&mut shortest, &mut path, Rc::clone(edge), &mut heap);
        }
    }

    (shortest, path)
}

fn relax(shortest: &mut Vec<f64>, path: &mut Vec<Option<usize>>, edge: Rc<DirectedEdge>, heap: &mut BinaryHeap<Reverse<Rc<DirectedEdge>>>) {
    let new_weight = shortest[edge.from()] + edge.weight();
    if new_weight < shortest[edge.to()] {
        shortest[edge.to()] = new_weight;
        path[edge.to()] = Some(edge.from());
        heap.push(Reverse(Rc::clone(&edge)));
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

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