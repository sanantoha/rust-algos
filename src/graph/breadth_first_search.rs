use crate::graph::EdgeWeightedDigraph;
use std::collections::VecDeque;


// O(n) time | O(n) space
pub fn bfs(graph: &EdgeWeightedDigraph, start: usize) -> Vec<usize> {

    let mut queue = VecDeque::with_capacity(graph.v);
    queue.push_back(start);

    let mut visited = vec![false; graph.v];

    let mut res = vec![];

    while let Some(v) = queue.pop_front() {

        if visited[v] {
            continue;
        }
        visited[v] = true;
        res.push(v);

        for edge in graph.adj(v) {
            queue.push_back(edge.to());
        }
    }

    res
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