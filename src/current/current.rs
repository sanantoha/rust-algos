use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::EdgeT;

pub fn mst(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    HashMap::new()
}

pub fn mst1(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, Vec<Rc<EdgeT<String>>>> {
    HashMap::new()
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