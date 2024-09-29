use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use crate::graph::EdgeT;

// O(E * log(V)) time | O(V) space
pub fn mst(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    let mut ngraph = HashMap::new();

    let mut heap = BinaryHeap::new();
    let start = String::from("0");
    for edge in graph.get(&start).unwrap_or(vec![].as_ref()) {
        heap.push(Reverse(Data { edge: Rc::clone(edge), from: start.to_owned() }));
    }

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut in_tree = 1;

    while let Some(Reverse(data)) = heap.pop() {
        if in_tree >= graph.len() {
            continue;
        }

        let from = data.from;
        let to = data.edge.other(&from).to_owned();

        if visited.contains(&to) {
            continue;
        }
        visited.insert(to.clone());
        in_tree += 1;
        let new_edge = Rc::new(EdgeT::new(data.edge.v.to_owned(), data.edge.u.to_owned(), data.edge.weight));
        let lst_from = ngraph.entry(from).or_insert_with(Vec::new);
        lst_from.push(Rc::clone(&new_edge));
        let lst_to = ngraph.entry(to.to_owned()).or_insert_with(Vec::new);
        lst_to.push(Rc::clone(&new_edge));

        for edge in graph.get(&to).unwrap_or(vec![].as_ref()) {
            heap.push(Reverse(Data { edge: Rc::clone(edge), from: to.to_owned() }));
        }
    }

    if in_tree != graph.len() {
        return HashMap::new();
    }

    ngraph
}

#[derive(Debug, Eq, PartialEq)]
struct Data {
    edge: Rc<EdgeT<String>>,
    from: String,
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.edge.weight.total_cmp(&other.edge.weight)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::graph::{compare_as_map, graph_to_string, EdgeT};
    use crate::graph::prim_min_spanning_tree_as_map::mst;
    /*
            6 5
            0: 0-1 7.0
            1: 1-2 3.0  0-1 7.0
            2: 1-2 3.0  2-4 3.0
            3: 3-4 2.0
            4: 3-4 2.0  4-5 2.0  2-4 3.0
            5: 4-5 2.0
         */
    #[test]
    fn test_mst() {
        let graph = create_graph();
        let graph_str = graph_to_string(&graph);
        println!("{}", graph_str);

        let mut exp_graph = HashMap::new();
        let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 7.0));
        let edge12 = Rc::new(EdgeT::new("1".to_owned(), "2".to_owned(), 3.0));
        let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 3.0));
        let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 2.0));
        let edge45 = Rc::new(EdgeT::new("4".to_owned(), "5".to_owned(), 2.0));
        exp_graph.insert(String::from("0"), vec![Rc::clone(&edge01)]);
        exp_graph.insert(String::from("1"), vec![Rc::clone(&edge12), Rc::clone(&edge01)]);
        exp_graph.insert(String::from("2"), vec![Rc::clone(&edge12), Rc::clone(&edge24)]);
        exp_graph.insert(String::from("3"), vec![Rc::clone(&edge34)]);
        exp_graph.insert(String::from("4"), vec![Rc::clone(&edge34), Rc::clone(&edge45), Rc::clone(&edge24)]);
        exp_graph.insert(String::from("5"), vec![Rc::clone(&edge45)]);

        let res = mst(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph)
        assert!(compare_as_map(&res, &exp_graph));
    }

    /*
        7 6
        0: 0-1 2.0  0-2 3.0
        1: 0-1 2.0  1-6 3.0
        2: 2-4 1.0  0-2 3.0
        3: 3-4 5.0
        4: 2-4 1.0  3-4 5.0
        5: 5-6 2.0
        6: 5-6 2.0  1-6 3.0
     */
    #[test]
    fn test_mst1() {
        let graph = create_graph1();
        let graph_str = graph_to_string(&graph);
        println!("{}", graph_str);

        let mut exp_graph = HashMap::new();
        let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 2.0));
        let edge02 = Rc::new(EdgeT::new("0".to_owned(), "2".to_owned(), 3.0));
        let edge16 = Rc::new(EdgeT::new("1".to_owned(), "6".to_owned(), 3.0));
        let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 1.0));
        let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 5.0));
        let edge56 = Rc::new(EdgeT::new("5".to_owned(), "6".to_owned(), 2.0));

        exp_graph.insert(String::from("0"), vec![Rc::clone(&edge01), Rc::clone(&edge02)]);
        exp_graph.insert(String::from("1"), vec![Rc::clone(&edge01), Rc::clone(&edge16)]);
        exp_graph.insert(String::from("2"), vec![Rc::clone(&edge02), Rc::clone(&edge24)]);
        exp_graph.insert(String::from("3"), vec![Rc::clone(&edge34)]);
        exp_graph.insert(String::from("4"), vec![Rc::clone(&edge24), Rc::clone(&edge34)]);
        exp_graph.insert(String::from("5"), vec![Rc::clone(&edge56)]);
        exp_graph.insert(String::from("6"), vec![Rc::clone(&edge16), Rc::clone(&edge56)]);

        let res = mst(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph);
        assert!(compare_as_map(&res, &exp_graph));
    }

    fn create_graph() -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
        let mut graph = HashMap::new();

        let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 7.0));
        let edge02 = Rc::new(EdgeT::new("0".to_owned(), "2".to_owned(), 8.0));

        let edge12 = Rc::new(EdgeT::new("1".to_owned(), "2".to_owned(), 3.0));
        let edge13 = Rc::new(EdgeT::new("1".to_owned(), "3".to_owned(), 6.0));

        let edge23 = Rc::new(EdgeT::new("2".to_owned(), "3".to_owned(), 4.0));
        let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 3.0));

        let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 2.0));
        let edge35 = Rc::new(EdgeT::new("3".to_owned(), "5".to_owned(), 5.0));

        let edge45 = Rc::new(EdgeT::new("4".to_owned(), "5".to_owned(), 2.0));

        graph.insert(String::from("0"), vec![Rc::clone(&edge01), Rc::clone(&edge02)]);
        graph.insert(String::from("1"), vec![Rc::clone(&edge01), Rc::clone(&edge12), Rc::clone(&edge13)]);
        graph.insert(String::from("2"), vec![Rc::clone(&edge02), Rc::clone(&edge12), Rc::clone(&edge23), Rc::clone(&edge24)]);
        graph.insert(String::from("3"), vec![Rc::clone(&edge13), Rc::clone(&edge23), Rc::clone(&edge34), Rc::clone(&edge35)]);
        graph.insert(String::from("4"), vec![Rc::clone(&edge24), Rc::clone(&edge34), Rc::clone(&edge45)]);
        graph.insert(String::from("5"), vec![Rc::clone(&edge35), Rc::clone(&edge45)]);

        graph
    }

    fn create_graph1() -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
        let mut graph = HashMap::new();

        /*
        7 10
        0: 0->1 2.0  0->2 3.0  0->3 7.0
        1: 0->1 2.0  1->2 6.0  1->6 3.0
        2: 0->2 3.0  1->2 6.0  2->4 1.0  2->5 8.0
        3: 0->3 7.0  3->4 5.0
        4: 2->4 1.0  3->4 5.0  4->5 4.0
        5: 2->5 8.0  4->5 4.0  5->6 2.0
        6: 1->6 3.0  5->6 2.0
         */

        let edge01 = Rc::new(EdgeT::new("0".to_owned(), "1".to_owned(), 2.0));
        let edge02 = Rc::new(EdgeT::new("0".to_owned(), "2".to_owned(), 3.0));
        let edge03 = Rc::new(EdgeT::new("0".to_owned(), "3".to_owned(), 7.0));

        let edge12 = Rc::new(EdgeT::new("1".to_owned(), "2".to_owned(), 6.0));
        let edge16 = Rc::new(EdgeT::new("1".to_owned(), "6".to_owned(), 3.0));

        let edge24 = Rc::new(EdgeT::new("2".to_owned(), "4".to_owned(), 1.0));
        let edge25 = Rc::new(EdgeT::new("2".to_owned(), "5".to_owned(), 8.0));

        let edge34 = Rc::new(EdgeT::new("3".to_owned(), "4".to_owned(), 5.0));

        let edge45 = Rc::new(EdgeT::new("4".to_owned(), "5".to_owned(), 4.0));

        let edge56 = Rc::new(EdgeT::new("5".to_owned(), "6".to_owned(), 2.0));

        graph.insert(String::from("0"), vec![Rc::clone(&edge01), Rc::clone(&edge02), Rc::clone(&edge03)]);
        graph.insert(String::from("1"), vec![Rc::clone(&edge01), Rc::clone(&edge12), Rc::clone(&edge16)]);
        graph.insert(String::from("2"), vec![Rc::clone(&edge02), Rc::clone(&edge12), Rc::clone(&edge24), Rc::clone(&edge25)]);
        graph.insert(String::from("3"), vec![Rc::clone(&edge03), Rc::clone(&edge34)]);
        graph.insert(String::from("4"), vec![Rc::clone(&edge24), Rc::clone(&edge34), Rc::clone(&edge45)]);
        graph.insert(String::from("5"), vec![Rc::clone(&edge25), Rc::clone(&edge45), Rc::clone(&edge56)]);
        graph.insert(String::from("6"), vec![Rc::clone(&edge16), Rc::clone(&edge56)]);

        graph
    }
}
