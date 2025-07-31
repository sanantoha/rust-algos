use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::EdgeT;

pub fn shortest_path(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, start: String) -> (HashMap<String, f64>, HashMap<String, String>) {
    (HashMap::new(), HashMap::new())
}

pub fn find_negative_weight_circle(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, shortest: &HashMap<String, f64>, prev: &HashMap<String, String>) -> Vec<String> {
    vec![]
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use itertools::Itertools;
    use crate::graph::{graph_from_file, graph_to_string};
    use super::{find_negative_weight_circle, shortest_path};

    #[test]
    fn test_shortest_path() {
        if let Ok(graph) = graph_from_file(PathBuf::from("src/graph/bellmanFord.txt")) {
            println!("graph: {}", graph_to_string(&graph));

            let (shortest, prev) = shortest_path(&graph, "0".to_string());

            let mut exp_prev = HashMap::new();
            exp_prev.insert(String::from("0"), String::from("4"));
            exp_prev.insert(String::from("1"), String::from("2"));
            exp_prev.insert(String::from("2"), String::from("4"));
            exp_prev.insert(String::from("3"), String::from("1"));
            exp_prev.insert(String::from("4"), String::from("1"));

            println!("shortest: {:?}, prev {:?}", shortest, prev);
            assert_eq!(prev.len(), exp_prev.len());
            for (k, v) in &prev {
                if let Some(v2) = exp_prev.get(k) {
                    assert_eq!(v, v2, "{}", k);
                }
            }

            let circle = find_negative_weight_circle(&graph, &shortest, &prev);
            println!("{:?}", circle);
            let circle_int: Vec<i32> = circle.into_iter().map(|x| x.parse::<i32>().unwrap_or_default()).sorted().collect();
            assert_eq!(circle_int, vec![1, 2, 4]);
        }
    }
}