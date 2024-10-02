use std::collections::VecDeque;
use crate::graph::Digraph;

// O(E + V) time | O(V) space
pub fn sort_rec(graph: &Digraph) -> Result<Vec<usize>, String> {
    let mut stack = vec![];
    let mut visited = vec![0u8; graph.v];

    for v in 0..graph.v {
        if visited[v] == 0 {
            if let Err(u) = dfs(graph, v, &mut visited, &mut stack) {
                return Err(format!("circle in the graph in {}", u));
            }
        }
    }

    let mut res = vec![];

    while let Some(v) = stack.pop() {
        res.push(v);
    }

    Ok(res)
}

fn dfs(graph: &Digraph, v: usize, visited: &mut Vec<u8>, stack: &mut Vec<usize>) -> Result<(), String> {
    visited[v] = 1;

    for &u in graph.adj(v) {
        if visited[u] == 1 {
            return Err(format!("cyrcle in the graph in {}", v));
        }
        if visited[u] == 0 {
            if let Err(k) = dfs(graph, u, visited, stack) {
                return Err(k);
            }
        }
    }

    visited[v] = 2;
    stack.push(v);
    Ok(())
}

// O(E + V) time | O(V) space
pub fn sort_iter(graph: &Digraph) -> Result<Vec<usize>, String> {

    let mut cnt = vec![0usize; graph.v];

    for v in 0..graph.v {
        for &u in graph.adj(v) {
            cnt[u] += 1;
        }
    }

    let mut is_circle = true;
    let mut queue = VecDeque::new();
    for v in 0..graph.v {
        if cnt[v] == 0 {
            queue.push_back(v);
            is_circle = false;
        }
    }

    if is_circle {
        return Err("circle in the graph".to_string());
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
        return Err("circle in the graph".to_string());
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::graph::Digraph;
    use crate::graph::topological_sort::{sort_iter, sort_rec};

    #[test]
    fn test_sort_rec() {

        if let Ok(graph) = Digraph::from_file(PathBuf::from("src/graph/digraph.txt")) {
            println!("{}", graph);

            let res = sort_rec(&graph).expect("sorting successfully");
            println!("{:?}", res);
            assert_eq!(res, vec![8, 9, 1, 0, 2, 3, 4, 5, 10, 11, 6, 7, 12, 13]);
        }
    }

    #[test]
    fn test_sort_iter() {

        if let Ok(graph) = Digraph::from_file(PathBuf::from("src/graph/digraph.txt")) {
            println!("{}", graph);

            let res = sort_iter(&graph).expect("sorting successfully");
            println!("{:?}", res);
            assert_eq!(res, vec![0, 1, 8, 2, 9, 4, 3, 5, 6, 10, 7, 11, 12, 13]);
        }
    }
}