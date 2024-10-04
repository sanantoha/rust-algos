use std::collections::{HashMap, HashSet, VecDeque};

// O(w * l ^ 2) time | O(w * l ^ 2) space
pub fn find_ladders<'a>(begin_word: &'a str, end_word: &'a str, word_list: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    if begin_word.is_empty() || end_word.is_empty() || word_list.is_empty() || !word_list.contains(&end_word) {
        return vec![];
    }

    let adj_list = create_adj_list(word_list);

    let mut queue = VecDeque::new();
    queue.push_back((begin_word, vec![begin_word]));

    let mut res = vec![];

    let mut visited = HashSet::new();
    visited.insert(begin_word);

    while !queue.is_empty() {
        let mut size = queue.len();

        let mut curr_visited = HashSet::new();

        while size > 0 {
            size -= 1;

            if let Some((curr_word, path)) = queue.pop_front() {

                if curr_word == end_word {
                    res.push(path);
                    continue;
                }

                for i in 0..curr_word.len() {
                    let key = get_key(curr_word, i);
                    for &word in adj_list.get(&key).unwrap_or(&vec![]) {
                        if visited.contains(&word) {
                            continue;
                        }
                        curr_visited.insert(word);
                        let mut new_path = path.clone();
                        new_path.push(word);
                        queue.push_back((word, new_path));
                    }
                }
            }
        }

        visited.extend(curr_visited);
    }

    res
}

fn create_adj_list<'a>(word_list: Vec<&'a str>) -> HashMap<String, Vec<&'a str>> {
    let mut adj_list: HashMap<String, Vec<&'a str>> = HashMap::new();

    for word in word_list {
        for i in 0..word.len() {
            let key = get_key(word, i);
            adj_list.entry(key).or_insert(vec![]).push(word);
        }
    }

    adj_list
}

fn get_key(word: &str, i: usize) -> String {
    format!("{}*{}", &word[0..i], &word[i+1..word.len()])
}


#[cfg(test)]
mod tests {
    use crate::graph::word_ladder_ii::find_ladders;

    #[test]
    fn test_find_ladders() {
        let begin_word = "hit";
        let end_word = "cog";
        let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"];

        let res = find_ladders(begin_word, end_word, word_list);
        println!("{:?}", res);

        assert_eq!(res, vec![vec!["hit", "hot", "dot", "dog", "cog"], vec!["hit", "hot", "lot", "log", "cog"]]);
    }
}