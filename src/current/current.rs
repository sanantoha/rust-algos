use crate::graph::Digraph;

pub fn sort_rec(graph: &Digraph) -> Result<Vec<usize>, String> {

    Err("Not implemented".to_owned())
}

pub fn sort_iter(graph: &Digraph) -> Result<Vec<usize>, String> {
    Err("Not implemented".to_owned())
}


#[cfg(test)]
mod tests {
    use super::{sort_iter, sort_rec};
    use crate::graph::Digraph;

    #[test]
    fn test_sort_rec() {
        let mut graph = Digraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        assert_eq!(sort_rec(&graph), Err("circle in the graph 4".to_owned()));
    }

    #[test]
    fn test_sort_rec_case1() {
        let mut graph = Digraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        assert_eq!(sort_rec(&graph), Ok(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_sort_iter() {
        let mut graph = Digraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        assert_eq!(sort_iter(&graph), Err("circle in the graph".to_owned()));
    }

    #[test]
    fn test_sort_iter_case1() {
        let mut graph = Digraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        assert_eq!(sort_iter(&graph), Ok(vec![0, 1, 2, 3, 4]));
    }
}