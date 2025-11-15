use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::EdgeT;

pub fn bfs(_graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, _start: String) -> Vec<String> {
    vec![]
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