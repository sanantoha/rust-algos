use std::collections::{HashMap, VecDeque};

// O(E + V) time | O(V) space
pub fn sort_rec(graph: &HashMap<String, Vec<String>>) -> Result<Vec<&str>, String> {
    let mut visited = HashMap::new();
    for v in graph.keys() {
        visited.insert(v.as_str(), 0);
    }

    let mut stack = vec![];

    for v in graph.keys() {
        if *visited.get(v.as_str()).unwrap_or(&0) == 0 {
            if let Err(err) = dfs(graph, &mut visited, v.as_str(), &mut stack) {
                return Err(err);
            }
        }
    }

    let mut res = vec![];

    while let Some(v) = stack.pop() {
        res.push(v);
    }

    Ok(res)
}

fn dfs<'a>(graph: &'a HashMap<String, Vec<String>>, visited: &mut HashMap<&'a str, i32>, v: &'a str, stack: &mut Vec<&'a str>) -> Result<(), String> {
    visited.insert(v, 1);

    if let Some(lst) = graph.get(v) {
        for u in lst {
            let i = *visited.get(u.as_str()).unwrap_or(&0);
            if i == 1 {
                return Err(format!("circle in the graph {}", u));
            }
            if i == 0 {
                if let Err(err) = dfs(graph, visited, u, stack) {
                    return Err(err);
                }
            }
        }
    }


    visited.insert(v, 2);
    stack.push(v);

    Ok(())
}

// O(E + V) time | O(V) space
pub fn sort_iter(graph: &HashMap<String, Vec<String>>) -> Result<Vec<&str>, String> {

    let mut cnt = HashMap::new();
    for v in graph.keys() {
        cnt.insert(v.as_str(), 0);
    }

    for v in graph.keys() {
        if let Some(lst) = graph.get(v) {
            for u in lst {
                cnt.entry(u.as_str()).and_modify(|c| *c += 1 ).or_insert(1);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut is_circle = true;

    for (&k, v) in cnt.iter() {
        if *v == 0 {
            queue.push_back(k);
            is_circle = false;
        }
    }

    if is_circle {
        return Err("circle in the graph".to_string());
    }

    let mut res = vec![];

    while let Some(v) = queue.pop_front() {
        res.push(v);

        if let Some(lst) = graph.get(v) {
            for u in lst {
                let c = cnt.entry(u.as_str()).or_insert(0);
                *c -= 1;
                if *c == 0 {
                    queue.push_back(u);
                }
            }
        }
    }

    if res.len() != graph.len() {
        return Err("circle in the graph".to_string());
    }

    Ok(res)
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