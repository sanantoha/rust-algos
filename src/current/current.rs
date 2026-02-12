use crate::graph::Digraph;


pub fn sort_rec(graph: &Digraph) -> Result<Vec<usize>, String> {
    Err("not impl".to_owned())
}

pub fn sort_iter(graph: &Digraph) -> Result<Vec<usize>, String> {
    Err("not impl".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{sort_iter, sort_rec};
    use crate::graph::Digraph;
    use std::path::PathBuf;

    #[test]
    fn test_sort_rec() {

        if let Ok(graph) = Digraph::from_file(PathBuf::from("src/graph/digraph.txt")) {
            println!("{}", graph);

            let res = sort_rec(&graph).expect("sorting successfully");
            println!("{:?}", res);
            assert_eq!(res, vec![8, 9, 1, 0, 2, 3, 4, 5, 10, 11, 6, 7, 12, 13]);
        }
    }

    #[test]
    fn test_sort_iter() {

        if let Ok(graph) = Digraph::from_file(PathBuf::from("src/graph/digraph.txt")) {
            println!("{}", graph);

            let res = sort_iter(&graph).expect("sorting successfully");
            println!("{:?}", res);
            assert_eq!(res, vec![0, 1, 8, 2, 9, 4, 3, 5, 6, 10, 7, 11, 12, 13]);
        }
    }
}