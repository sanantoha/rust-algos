use crate::graph::EdgeT;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

// O(E * log(E)) time | O(E + V) space
pub fn mst(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut ngraph = HashMap::new();
    let edges = make_edges(graph);

    let mut heap = BinaryHeap::new();
    for edge in &edges {
        heap.push(Reverse(Rc::clone(edge)));
    }

    let mut ranks: HashMap<String, i32> = HashMap::new();
    for v in graph.keys() {
        ranks.insert(v.to_owned(), 0);
    }
    let mut parents = make_set(graph);

    while let Some(Reverse(min_edge)) = heap.pop() {
        let from = min_edge.from();
        let to = min_edge.to();

        let p_from = find(&mut parents, from.to_owned());
        let p_to = find(&mut parents, to.to_owned());

        if p_from != p_to {
            union(&mut parents, &mut ranks, p_from, p_to);
            let new_edge = Rc::new(EdgeT::new(from.clone(), to.clone(), min_edge.weight));
            ngraph.entry(from.to_owned()).or_insert(Vec::new()).push(Rc::clone(&new_edge));
            ngraph.entry(to.to_owned()).or_insert(Vec::new()).push(Rc::clone(&new_edge));
        }
    }

    ngraph
}

// O(E * log(E)) time | O(E + V) space
pub fn mst1(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut ngraph = HashMap::new();
    let edges = make_edges(graph);

    let mut edge_list = vec![];

    for edge in &edges {
        edge_list.push(Rc::clone(edge));
    }

    edge_list.sort();

    let mut ranks: HashMap<String, i32> = HashMap::new();
    for v in graph.keys() {
        ranks.insert(v.to_owned(), 0);
    }
    let mut parents = make_set(graph);

    for min_edge in edge_list {
        let from = min_edge.from();
        let to = min_edge.to();

        let p_from = find(&mut parents, from.to_owned());
        let p_to = find(&mut parents, to.to_owned());

        if p_from != p_to {
            union(&mut parents, &mut ranks, p_from, p_to);
            let new_edge = Rc::new(EdgeT::new(from.clone(), to.clone(), min_edge.weight));
            ngraph.entry(from.to_owned()).or_insert(Vec::new()).push(Rc::clone(&new_edge));
            ngraph.entry(to.to_owned()).or_insert(Vec::new()).push(Rc::clone(&new_edge));
        }
    }

    ngraph
}

fn union(parents: &mut HashMap<String, String>, ranks: &mut HashMap<String, i32>, p_from: String, p_to: String) {
    let rank_from = ranks.get(p_from.as_str()).unwrap_or(&0);
    let rank_to = ranks.get(p_to.as_str()).unwrap_or(&0);
    if rank_from < rank_to {
        parents.insert(p_from.clone(), p_to.clone());
    } else if rank_from > rank_to {
        parents.insert(p_to.clone(), p_from.clone());
    } else {
        parents.insert(p_from.clone(), p_to.clone());
        ranks.insert(p_to.clone(), ranks.get(p_to.as_str()).unwrap_or(&0) + 1);
    }
}

fn find(parents: &mut HashMap<String, String>, v: String) -> String {
    let empty_string = String::new();
    if *parents.get(&v).unwrap_or(&empty_string) == v {
        return v;
    }

    let nv = parents.get(&v).unwrap_or(&empty_string);
    let res = find(parents, nv.clone());
    parents.insert(v, res.clone());
    res
}

fn make_edges(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashSet<Rc<EdgeT<String>>> {
    let mut edges = HashSet::new();

    for lst in graph.values() {
        for edge in lst {
            edges.insert(Rc::clone(edge));
        }
    }

    edges
}

// fn make_set1<'a>(graph: &'a HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<&'a str, &'a str> {
//     let mut parents: HashMap<&str, &str> = HashMap::new();
//     for v in graph.keys() {
//         parents.insert(v, v);
//     }
//     parents
// }

fn make_set(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, String> {
    let mut parents = HashMap::new();
    for v in graph.keys() {
        parents.insert(v.to_owned(), v.to_owned());
    }
    parents
}


#[cfg(test)]
mod tests {
    use super::{mst, mst1};
    use crate::graph::{compare_as_map, create_graph, create_graph1, exp_graph, exp_graph1, graph_to_string};

    /*
        6 5
        0: 0-1 7.00000
        1: 1-2 3.00000  0-1 7.00000
        2: 1-2 3.00000  2-4 3.00000
        3: 3-4 2.00000
        4: 3-4 2.00000  4-5 2.00000  2-4 3.00000
        5: 4-5 2.00000
    */
    #[test]
    fn test_mst() {
        let graph = create_graph();
        let graph_str = graph_to_string(&graph);
        println!("{}", graph_str);

        let exp_graph = exp_graph();

        let res = mst(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph)
        assert!(compare_as_map(&res, &exp_graph));
    }

    #[test]
    fn test_mst_case1() {
        let graph = create_graph1();
        let graph_str = graph_to_string(&graph);
        println!("{}", graph_str);

        let exp_graph = exp_graph1();

        let res = mst(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph)
        assert!(compare_as_map(&res, &exp_graph));
    }

    #[test]
    fn test_mst1() {
        let graph = create_graph();
        let graph_str = graph_to_string(&graph);
        println!("{}", graph_str);

        let exp_graph = exp_graph();

        let res = mst1(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph)
        assert!(compare_as_map(&res, &exp_graph));
    }

    #[test]
    fn test_mst1_case1() {
        let graph = create_graph1();
        let graph_str = graph_to_string(&graph);
        println!("{}", graph_str);

        let exp_graph = exp_graph1();

        let res = mst1(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph)
        assert!(compare_as_map(&res, &exp_graph));
    }

}