use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use crate::graph::EdgeT;

// O((E + V) * log(V)) time | O(V) space
pub fn shortest_paths(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, start: String) -> (HashMap<String, f64>, HashMap<String, String>) {
    let mut shortest = HashMap::new();
    let mut prev = HashMap::new();
    for v in graph.keys() {
        shortest.insert(v.to_string(), f64::MAX);
        prev.insert(v.to_string(), "".to_string());
    }
    shortest.insert(start.clone(), 0.0);

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Rc::new(EdgeT::new(start.clone(), start.clone(), 0f64))));

    while let Some(Reverse(min_edge)) = heap.pop() {
        if let Some(lst) = graph.get(min_edge.to()) {
            for edge in lst.iter() {
                relax(&mut shortest, &mut prev, Rc::clone(edge), &mut heap);
            }
        }
    }

    (shortest, prev)
}

fn relax(shortest: &mut HashMap<String, f64>, prev: &mut HashMap<String, String>, edge: Rc<EdgeT<String>>, heap: &mut BinaryHeap<Reverse<Rc<EdgeT<String>>>>) {
    let new_weight = *shortest.get(edge.from()).unwrap_or(&f64::MAX) + edge.weight();
    let curr_weight = *shortest.get(edge.to()).unwrap_or(&f64::MAX);

    if new_weight < curr_weight {
        shortest.insert(edge.to().to_string(), new_weight);
        prev.insert(edge.to().to_string(), edge.from().to_string());
        heap.push(Reverse(Rc::clone(&edge)));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::graph::{graph_from_file, graph_to_string};
    use crate::graph::dijkstra_shortest_paths_as_map::shortest_paths;

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