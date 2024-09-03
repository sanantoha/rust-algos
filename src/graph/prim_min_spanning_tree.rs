
use super::EdgeWeightedGraph;

pub fn mst(graph: &EdgeWeightedGraph) -> Option<EdgeWeightedGraph> {
    None
}

#[cfg(test)]
mod tests {

    use crate::graph::Edge;

    use super::EdgeWeightedGraph;

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

        println!("{}", graph);
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

        println!("{}", graph);
    }
}