use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::graph::EdgeT;

pub fn dfs_rec<'a>(graph: &'a HashMap<String, Vec<Rc<EdgeT<String>>>>, start: &'a str) -> Vec<&'a str> {
    vec![]
}


pub fn dfs<'a>(graph: &'a HashMap<String, Vec<Rc<EdgeT<String>>>>, start: &'a str) -> Vec<&'a str> {
    vec![]
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::{dfs_rec, dfs};
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