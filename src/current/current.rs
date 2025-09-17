use std::collections::HashMap;

pub fn sort_rec(graph: &HashMap<String, Vec<String>>) -> Result<Vec<&str>, String> {
    Err("not found".to_owned())
}

pub fn sort_iter(graph: &HashMap<String, Vec<String>>) -> Result<Vec<&str>, String> {
    Err("not found".to_owned())
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{sort_rec, sort_iter};

    #[test]
    fn test_sort_rec() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string(), "D".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec![]);

        let res = sort_rec(&graph);

        assert_eq!(res, Ok(vec!["A", "B", "C", "D"]));
    }

    #[test]
    fn test_sort_rec_case1() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string(), "D".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec!["A".to_string()]);

        let res = sort_rec(&graph);

        println!("{:?}", res);
        assert!(res.is_err());
        if let Err(msg) = res {
            assert!(msg.contains("circle in the graph "));
        }
    }

    #[test]
    fn test_sort_iter() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string(), "D".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec![]);

        let res = sort_iter(&graph);

        assert_eq!(res, Ok(vec!["A", "B", "C", "D"]));
    }

    #[test]
    fn test_sort_iter_case1() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string(), "D".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec!["B".to_string()]);

        let res = sort_iter(&graph);

        assert_eq!(res, Err("circle in the graph".to_string()));
    }
}