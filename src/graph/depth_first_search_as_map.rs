use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::graph::EdgeT;

// O(E + V) time | O(V) space
pub fn dfs_rec<'a>(graph: &'a HashMap<String, Vec<Rc<EdgeT<String>>>>, start: &'a str) -> Vec<&'a str> {

    let mut res = vec![];
    let mut visited = HashSet::new();

    recursion(graph, start, &mut visited, &mut res);

    res
}

fn recursion<'a>(graph: &'a HashMap<String, Vec<Rc<EdgeT<String>>>>, v: &'a str, visited: &mut HashSet<&'a str>, res: &mut Vec<&'a str>) {
    if visited.contains(v) {
        return;
    }
    visited.insert(v);
    res.push(v);

    if let Some(lst) = graph.get(v) {
        for edge in lst.iter() {
            recursion(graph, edge.to(), visited, res);
        }
    }
}

// O(E + V) time | O(V) space
pub fn dfs<'a>(graph: &'a HashMap<String, Vec<Rc<EdgeT<String>>>>, start: &'a str) -> Vec<&'a str> {

    let mut res = vec![];

    let mut stack = vec![];
    stack.push(start);

    let mut visited = HashSet::new();

    while let Some(v) = stack.pop() {
        if visited.contains(v) {
            continue;
        }
        visited.insert(v);
        res.push(v);

        if let Some(lst) = graph.get(v) {
            for edge in lst.iter().rev() {
                stack.push(edge.to());
            }
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::graph::depth_first_search_as_map::dfs_rec;
    use crate::graph::depth_first_search_as_map::dfs;
    use crate::graph::{graph_from_file, graph_to_string};

    const PATH: &str = "src/graph/dfs.txt";

    const EXP: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14"];

    #[test]
    fn it_dfs_rec() {
        if let Ok(graph) = graph_from_file(PathBuf::from(PATH)) {
            let graph_str = graph_to_string(&graph);
            println!("{}", graph_str);

            let res = dfs_rec(&graph, "0");
            println!("{:?}", res);
            assert_eq!(res, EXP)
        }
    }

    #[test]
    fn it_dfs() {
        if let Ok(graph) = graph_from_file(PathBuf::from(PATH)) {
            let graph_str = graph_to_string(&graph);
            println!("{}", graph_str);

            let res = dfs(&graph, "0");
            println!("{:?}", res);
            assert_eq!(res, EXP)
        }
    }
}