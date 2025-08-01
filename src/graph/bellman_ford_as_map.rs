use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use itertools::Itertools;
use crate::graph::EdgeT;

// O(E * V) time | O(V) space
pub fn shortest_path(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, start: String) -> (HashMap<String, f64>, HashMap<String, String>) {
    let mut shortest = HashMap::new();
    let mut prev = HashMap::new();
    let mut edges = HashSet::new();
    for v in graph.keys() {
        shortest.insert(v.clone(), f64::MAX);
        prev.insert(v.clone(), "".to_string());
        let lst = graph.get(v).unwrap_or(&vec![]).iter()
            .map(|x| Rc::clone(&x)).collect::<HashSet<_>>();
        edges.extend(lst);
    }
    let s = shortest.entry(start).or_insert(f64::MAX);
    *s = 0f64;

    // not required for this algorithms, but for stability
    let edges_list: Vec<Rc<EdgeT<String>>> = edges.iter().map(|x| Rc::clone(x)).sorted().collect();

    for _ in 0..(graph.len() - 1) {
        for edge in edges_list.iter() {
            relax(&mut shortest, &mut prev, Rc::clone(edge));
        }
    }

    (shortest, prev)
}

fn relax(shortest: &mut HashMap<String, f64>, prev: &mut HashMap<String, String>, edge: Rc<EdgeT<String>>) {
    let new_weight = *shortest.get(edge.from()).unwrap_or(&f64::MAX) + edge.weight();
    let curr_weight = *shortest.get(edge.to()).unwrap_or(&f64::MAX);
    if new_weight < curr_weight {
        shortest.insert(edge.to().into(), new_weight);
        prev.insert(edge.to().into(), edge.from().to_owned());
    }
}

// O(E + V) time | O(V) space
pub fn find_negative_weight_circle(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>, shortest: &HashMap<String, f64>, prev: &HashMap<String, String>) -> Vec<String> {
    let mut v = None;

    let mut edges = HashSet::new();
    for v in graph.keys() {
        if let Some(lst) = graph.get(v) {
            for edge in lst.iter() {
                edges.insert(Rc::clone(edge));
            }
        }
    }

    for edge in edges.iter() {
        let new_weight = *shortest.get(edge.from()).unwrap_or(&f64::MAX) + edge.weight();
        let curr_weight = *shortest.get(edge.to()).unwrap_or(&f64::MAX);
        if new_weight < curr_weight {
            v = Some(edge.from().to_owned());
            break;
        }
    }

    if v.is_none() {
        return vec![];
    }

    let mut res = vec![];

    if let Some(vi) = v {
        res.push(vi.to_owned());
        let mut u = prev.get(&vi);

        while let Some(ui) = u.take() {
            if vi == ui.to_owned() {
                break;
            }
            res.push(ui.to_owned());
            u = prev.get(ui);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use itertools::Itertools;
    use crate::graph::{graph_from_file, graph_to_string};
    use crate::graph::bellman_ford_as_map::{find_negative_weight_circle, shortest_path};

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