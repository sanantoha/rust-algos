
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::cmp::Ordering;
use std::cmp::Reverse;

use super::{Edge, EdgeWeightedGraph};

// O(E * log(V)) time | O(V) space
pub fn mst(graph: &EdgeWeightedGraph) -> Option<EdgeWeightedGraph> {
    let mut ngraph = EdgeWeightedGraph::new(graph.v);

    let start: usize = 0;

    let mut heap = BinaryHeap::new();
    for edge in graph.adj(start) {
        heap.push(Reverse(Data { edge: Rc::clone(edge), from: start }));
    }

    let mut visited = vec![false; graph.v];
    visited[start] = true;

    let mut in_tree = 1;
    
    while let Some(Reverse(Data { edge: min_edge, from })) = heap.pop() {
        if in_tree >= graph.v {
            break;
        }

        let to = min_edge.other(from);

        if visited[to] {
            continue;
        }
        visited[to] = true;
        in_tree += 1;
        ngraph.add_edge(Edge::new(min_edge.either(), min_edge.other(min_edge.either()), min_edge.weight));

        for edge in graph.adj(to) {
            heap.push(Reverse(Data { edge: Rc::clone(edge), from: to }));
        }
    }

    if in_tree < graph.v {
        return None;
    }


    Some(ngraph)
}

#[derive(Debug, Eq, PartialEq)]
struct Data {
    edge: Rc<Edge>,
    from: usize,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {        
        self.edge.weight.total_cmp(&other.edge.weight)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {

    use crate::graph::Edge;
    use crate::graph::EdgeWeightedGraph;
    use super::mst;

    #[test]
    fn it_mst() {
        /*
        6 5
        0: 0-1 7.00000
        1: 1-2 3.00000  0-1 7.00000
        2: 1-2 3.00000  2-4 3.00000
        3: 3-4 2.00000
        4: 3-4 2.00000  4-5 2.00000  2-4 3.00000
        5: 4-5 2.00000
        */

        let mut graph = EdgeWeightedGraph::new(6);
        graph.add_edge(Edge::new(0, 1, 7.0));
        graph.add_edge(Edge::new(0, 2, 8.0));
        graph.add_edge(Edge::new(1, 2, 3.0));
        graph.add_edge(Edge::new(1, 3, 6.0));
        graph.add_edge(Edge::new(2, 3, 4.0));
        graph.add_edge(Edge::new(2, 4, 3.0));
        graph.add_edge(Edge::new(3, 4, 2.0));
        graph.add_edge(Edge::new(3, 5, 5.0));
        graph.add_edge(Edge::new(4, 5, 2.0));

        println!("original:\n{}", graph);

        let mut exp_graph = EdgeWeightedGraph::new(6);
        exp_graph.add_edge(Edge::new(0, 1, 7.0));
        exp_graph.add_edge(Edge::new(1, 2, 3.0));
        exp_graph.add_edge(Edge::new(2, 4, 3.0));
        exp_graph.add_edge(Edge::new(3, 4, 2.0));
        exp_graph.add_edge(Edge::new(4, 5, 2.0));

        println!("expected:\n{}", exp_graph);

        let res_graph = mst(&graph).expect("should return graph");
        println!("result:\n{}", &res_graph);

        assert_eq!(res_graph, exp_graph);
    }

    #[test]
    fn it_mst_case2() {

        /*        
        7 6
        0: 0-1 2.00000  0-2 3.00000
        1: 0-1 2.00000  1-6 3.00000
        2: 2-4 1.00000  0-2 3.00000
        3: 3-4 5.00000
        4: 2-4 1.00000  3-4 5.00000
        5: 5-6 2.00000
        6: 5-6 2.00000  1-6 3.00000
         
         */

        let mut graph = EdgeWeightedGraph::new(7);
        graph.add_edge(Edge::new(0, 1, 2.0));
        graph.add_edge(Edge::new(0, 2, 3.0));
        graph.add_edge(Edge::new(0, 3, 7.0));
        graph.add_edge(Edge::new(1, 2, 6.0));
        graph.add_edge(Edge::new(1, 6, 3.0));
        graph.add_edge(Edge::new(2, 4, 1.0));
        graph.add_edge(Edge::new(2, 5, 8.0));
        graph.add_edge(Edge::new(5, 6, 2.0));
        graph.add_edge(Edge::new(3, 4, 5.0));
        graph.add_edge(Edge::new(4, 5, 4.0));

        println!("original:\n{}", graph);

        let mut exp_graph = EdgeWeightedGraph::new(7);
        exp_graph.add_edge(Edge::new(0, 1, 2.0));
        exp_graph.add_edge(Edge::new(0, 2, 3.0));
        exp_graph.add_edge(Edge::new(1, 6, 3.0));
        exp_graph.add_edge(Edge::new(2, 4, 1.0));
        exp_graph.add_edge(Edge::new(5, 6, 2.0));
        exp_graph.add_edge(Edge::new(3, 4, 5.0));

        println!("expected:\n{}", exp_graph);

        let res_graph = mst(&graph).expect("should return graph");
        println!("result:\n{}", &res_graph);

        assert_eq!(res_graph, exp_graph);
    }
}