use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use crate::graph::EdgeT;

// O(E + V) time | O(V) space
pub fn bfs(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, start: String) -> Vec<String> {
    if graph.is_empty() {
        return vec![];
    }

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = HashSet::new();

    let mut res = vec![];

    while let Some(v) = queue.pop_front() {
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v.clone());

        if let Some(edges) = graph.get(&v) {
            for edge in edges {
                queue.push_back(edge.to().to_owned());
            }
        }

        res.push(v);
    }

    res
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::bfs;
    use crate::graph::{graph_from_file, graph_to_string};


    #[test]
    fn it_bfs() {
        if let Ok(graph) = graph_from_file(PathBuf::from("src/graph/bfs.txt")) {
            let graph_str = graph_to_string(&graph);
            println!("{}", graph_str);

            let exp: Vec<String> = vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
            ].iter().map(|s| s.to_string()).collect();

            let res = bfs(&graph, "0".to_owned());
            println!("{:?}", res);
            assert_eq!(res, exp);
        }
    }
}