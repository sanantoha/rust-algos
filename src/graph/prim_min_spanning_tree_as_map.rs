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
    use crate::graph::{compare_as_map, create_graph, create_graph1, exp_graph, exp_graph1, graph_to_string};
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

        let exp_graph = exp_graph();

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

        let exp_graph = exp_graph1();

        let res = mst(&graph);
        println!("{}", graph_to_string(&exp_graph));
        println!("{}", graph_to_string(&res));

        // assert_eq!(res, exp_graph);
        assert!(compare_as_map(&res, &exp_graph));
    }

}
