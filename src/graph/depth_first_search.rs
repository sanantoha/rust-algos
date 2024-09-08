use std::collections::VecDeque;
use std::rc::Rc;
use crate::graph::DirectedEdge;
use crate::graph::EdgeWeightedDigraph;

// O(n) time | O(h) space
pub fn dfs_rec(graph: &EdgeWeightedDigraph, start: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut visited = vec![false; graph.v];
    recursive(graph, start, &mut visited, &mut res);
    res
}

fn recursive(graph: &EdgeWeightedDigraph, v: usize, visited: &mut Vec<bool>, res: &mut Vec<usize>) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    res.push(v);

    for edge in graph.adj(v) {
        recursive(graph, edge.to(), visited, res);
    }
}

// O(n) time | O(h) space
pub fn dfs(graph: &EdgeWeightedDigraph, start: usize) -> Vec<usize> {

    let mut res = vec![];

    let mut stack = VecDeque::new();
    stack.push_back(start);

    let mut visited = vec![false; graph.v];

    while let Some(v) = stack.pop_back() {

        if visited[v] {
            continue;
        }
        visited[v] = true;
        res.push(v);

        let mut edges: Vec<Rc<DirectedEdge>> = vec![];
        graph.adj(v).for_each(|x| edges.push(Rc::clone(x)) );

        for edge in edges.iter().rev() {
            stack.push_back(edge.to())
        }
    }
    
    res
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::graph::depth_first_search::dfs_rec;
    use crate::graph::depth_first_search::dfs;
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