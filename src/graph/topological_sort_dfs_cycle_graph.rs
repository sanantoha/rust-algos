use std::collections::VecDeque;
use crate::graph::Digraph;

// O(E + V) time | O(V) space
pub fn sort_rec(graph: &Digraph) -> Result<Vec<usize>, String> {

    let mut visited = vec![0u8; graph.v];
    let mut stack = vec![];

    for v in 0..graph.v {
        if visited[v] == 0 {
            if let Err(s) = dfs(graph, &mut visited, v, &mut stack) {
                return Err(s);
            }
        }
    }

    let mut res = vec![];
    while let Some(v) = stack.pop() {
        res.push(v);
    }
    Ok(res)
}

fn dfs(graph: &Digraph, visited: &mut Vec<u8>, v: usize, stack: &mut Vec<usize>) -> Result<(), String> {
    visited[v] = 1;

    for &u in graph.adj(v) {
        if visited[u] == 1 {
            return Err(format!("circle in the graph {}", v));
        }
        if visited[u] == 0 {
            if let Err(s) = dfs(graph, visited, u, stack) {
                return Err(s);
            }
        }
    }

    visited[v] = 2;
    stack.push(v);
    Ok(())
}

// O(E + V) time | O(V) space
pub fn sort_iter(graph: &Digraph) -> Result<Vec<usize>, String> {

    let mut cnt = vec![0; graph.v];

    for v in 0..graph.v {
        for &u in graph.adj(v) {
            cnt[u] += 1;
        }
    }

    let mut is_circle = true;
    let mut queue = VecDeque::new();
    for v in 0..graph.v {
        if cnt[v] == 0 {
            is_circle = false;
            queue.push_back(v);
        }
    }

    if is_circle {
        return Err("circle in the graph".to_owned());
    }

    let mut res = vec![];

    while let Some(v) = queue.pop_front() {
        res.push(v);

        for &u in graph.adj(v) {
            cnt[u] -= 1;

            if cnt[u] == 0 {
                queue.push_back(u);
            }
        }
    }

    if res.len() != graph.v {
        return Err("circle in the graph".to_owned());
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::graph::Digraph;
    use super::{sort_iter, sort_rec};

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