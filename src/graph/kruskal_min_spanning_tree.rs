use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;
use crate::graph::{Edge, EdgeWeightedGraph};

// O(E * log(E)) time | O(V + E) space
pub fn mst(graph: &EdgeWeightedGraph) -> Option<EdgeWeightedGraph> {
    let mut ngraph = EdgeWeightedGraph::new(graph.v);

    let mut heap = BinaryHeap::new();
    for edge in graph.edges() {
        heap.push(Reverse(Rc::clone(&edge)));
    }

    let mut parents = make_set(graph.v);
    let mut ranks = vec![0usize; graph.v];

    while let Some(Reverse(edge)) = heap.pop() {
        let from = edge.either();
        let to = edge.other(from);

        let p_from = find(&mut parents, from);
        let p_to = find(&mut parents, to);

        if p_from != p_to {
            union(&mut parents, &mut ranks, p_to, p_from);
            ngraph.add_edge(Edge::new(from, to, edge.weight))
        }
    }

    Some(ngraph)
}

// O(E * log(E)) time | O(V + E) space
pub fn mst1(graph: &EdgeWeightedGraph) -> Option<EdgeWeightedGraph> {
    let mut ngraph = EdgeWeightedGraph::new(graph.v);

    let mut edge_list = Vec::with_capacity(ngraph.v);
    for edge in graph.edges() {
        edge_list.push(Rc::clone(&edge));
    }

    edge_list.sort();

    let mut parents = make_set(graph.v);
    let mut ranks = vec![0usize; graph.v];

    for edge in edge_list {
        let from = edge.either();
        let to = edge.other(from);

        let p_from = find(&mut parents, from);
        let p_to = find(&mut parents, to);

        if p_from != p_to {
            union(&mut parents, &mut ranks, p_to, p_from);
            ngraph.add_edge(Edge::new(from, to, edge.weight))
        }
    }


    Some(ngraph)
}

fn make_set(v: usize) -> Vec<usize> {
    let mut parents = vec![0usize; v];
    for i in 0..v {
        parents[i] = i;
    }
    parents
}

fn find(parents: &mut Vec<usize>, v: usize) -> usize {
    if parents[v] == v {
        return v;
    }

    parents[v] = find(parents, parents[v]);
    parents[v]
}

fn union(parents: &mut Vec<usize>, rank: &mut Vec<usize>, v: usize, u: usize) {
    if rank[v] > rank[u] {
        parents[u] = v;
    } else if rank[v] < rank[u] {
        parents[v] = u;
    } else {
        parents[v] = u;
        rank[u] += 1;
    }
}

#[cfg(test)]
mod tests {

    use crate::graph::{compare, Edge};
    use crate::graph::EdgeWeightedGraph;
    use super::{mst, mst1};

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

        let graph = create_graph();
        println!("original:\n{}", graph);

        let exp_graph = create_exp_graph();
        println!("expected:\n{}", exp_graph);

        let res_graph = mst(&graph).expect("should return graph");
        println!("result:\n{}", &res_graph);

        // assert_eq!(res_graph, exp_graph);
        assert!(compare(&res_graph, &exp_graph));
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
        let graph = create_graph1();
        println!("original:\n{}", graph);

        let exp_graph = create_exp_graph1();
        println!("expected:\n{}", exp_graph);

        let res_graph = mst(&graph).expect("should return graph");
        println!("result:\n{}", &res_graph);

        // assert_eq!(res_graph, exp_graph);
        assert!(compare(&res_graph, &exp_graph));
    }

    #[test]
    fn it_mst1() {
        /*
        6 5
        0: 0-1 7.00000
        1: 1-2 3.00000  0-1 7.00000
        2: 1-2 3.00000  2-4 3.00000
        3: 3-4 2.00000
        4: 3-4 2.00000  4-5 2.00000  2-4 3.00000
        5: 4-5 2.00000
        */

        let graph = create_graph();
        println!("original:\n{}", graph);

        let exp_graph = create_exp_graph();
        println!("expected:\n{}", exp_graph);

        let res_graph = mst1(&graph).expect("should return graph");
        println!("result:\n{}", &res_graph);

        // assert_eq!(res_graph, exp_graph);
        assert!(compare(&res_graph, &exp_graph));
    }

    #[test]
    fn it_mst1_case2() {

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
        let graph = create_graph1();
        println!("original:\n{}", graph);

        let exp_graph = create_exp_graph1();
        println!("expected:\n{}", exp_graph);

        let res_graph = mst1(&graph).expect("should return graph");
        println!("result:\n{}", &res_graph);

        // assert_eq!(res_graph, exp_graph);
        assert!(compare(&res_graph, &exp_graph));
    }

    fn create_exp_graph1() -> EdgeWeightedGraph {
        let mut exp_graph = EdgeWeightedGraph::new(7);
        exp_graph.add_edge(Edge::new(0, 1, 2.0));
        exp_graph.add_edge(Edge::new(0, 2, 3.0));
        exp_graph.add_edge(Edge::new(1, 6, 3.0));
        exp_graph.add_edge(Edge::new(2, 4, 1.0));
        exp_graph.add_edge(Edge::new(5, 6, 2.0));
        exp_graph.add_edge(Edge::new(3, 4, 5.0));
        exp_graph
    }

    fn create_graph1() -> EdgeWeightedGraph {
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
        graph
    }

    fn create_exp_graph() -> EdgeWeightedGraph {
        let mut exp_graph = EdgeWeightedGraph::new(6);
        exp_graph.add_edge(Edge::new(0, 1, 7.0));
        exp_graph.add_edge(Edge::new(1, 2, 3.0));
        exp_graph.add_edge(Edge::new(2, 4, 3.0));
        exp_graph.add_edge(Edge::new(3, 4, 2.0));
        exp_graph.add_edge(Edge::new(4, 5, 2.0));
        exp_graph
    }

    fn create_graph() -> EdgeWeightedGraph {
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
        graph
    }

}