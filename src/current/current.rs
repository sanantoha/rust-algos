use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::EdgeT;


pub fn shortest_paths(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, start: String) -> (HashMap<String, f64>, HashMap<String, String>) {

    (HashMap::default(), HashMap::default())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::graph::{graph_from_file, graph_to_string};
    use super::shortest_paths;

    // {"0": 0.0, "1": 5.0, "4": 7.0, "3": 4.0, "2": 8.0}, {"1": "3", "4": "3", "0": "", "2": "1", "3": "0"}
    #[test]
    fn test_shortest_paths() {
        if let Ok(graph) = graph_from_file(PathBuf::from("src/graph/dijkstraShortestPath.txt")) {
            println!("{}", graph_to_string(&graph));

            let mut exp_prev = HashMap::new();
            exp_prev.insert(String::from("0"), String::from(""));
            exp_prev.insert(String::from("1"), String::from("3"));
            exp_prev.insert(String::from("2"), String::from("1"));
            exp_prev.insert(String::from("3"), String::from("0"));
            exp_prev.insert(String::from("4"), String::from("3"));

            let mut exp_shortest = HashMap::new();
            exp_shortest.insert(String::from("0"), 0.0);
            exp_shortest.insert(String::from("1"), 5.0);
            exp_shortest.insert(String::from("2"), 8.0);
            exp_shortest.insert(String::from("3"), 4.0);
            exp_shortest.insert(String::from("4"), 7.0);

            let (shortest, prev) = shortest_paths(&graph, "0".to_string());
            println!("{:?}", shortest);
            println!("{:?}", prev);

            assert_eq!(prev.len(), exp_prev.len());
            for (k, v) in &prev {
                if let Some(v2) = exp_prev.get(k) {
                    assert_eq!(v, v2, "{}", k);
                }
            }

            assert_eq!(shortest.len(), exp_shortest.len());
            for (k, v) in &shortest {
                if let Some(v2) = exp_shortest.get(k) {
                    assert_eq!(v, v2, "{}", k);
                }
            }
        }
    }
}