use std::collections::HashMap;
use std::rc::Rc;
use crate::graph::EdgeT;

pub fn mst(graph: &HashMap<String, Vec<Rc<EdgeT<String>>>>) -> HashMap<String, Vec<Rc<EdgeT<String>>>> {

    HashMap::default()
}


#[cfg(test)]
mod tests {
    use crate::graph::{compare_as_map, create_graph, create_graph1, exp_graph, exp_graph1, graph_to_string};
    use super::mst;
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