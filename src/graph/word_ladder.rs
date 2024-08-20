use std::collections::{HashMap, HashSet, VecDeque};


// O(n * l ^ 2) time | O(n * l ^ 2) space
pub fn ladder_length(begin_word: &str, end_word: &str, word_list: &[&str]) -> i32 {
    if begin_word.is_empty() || end_word.is_empty() || word_list.is_empty() || !word_list.contains(&end_word) {
        return 0;
    }

    let adj_list = create_adj_list(word_list);

    let mut queue = VecDeque::new();
    queue.push_back((begin_word, 1));

    let mut visited = HashSet::new();

    while !queue.is_empty() {
        if let Some((curr_word, level)) = queue.pop_front() {
            if curr_word == end_word {
                return level;
            }

            if visited.contains(curr_word) {
                continue;
            }
            visited.insert(curr_word);
    
            for i in 0..curr_word.len() {
                let key = format!("{}*{}", &curr_word[..i], &curr_word[i + 1..]);
    
                if let Some(lst) =  adj_list.get(&key) {
                    for &word in lst {
                        queue.push_back((word, level + 1));
                    }
                }
                
            }
        }
        
    }
    0
}

fn create_adj_list<'a>(word_list: &'a [&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut adj_list = HashMap::new();

    for &word in word_list.iter() {
        for i in 0..word.len() {
            let key = format!("{}*{}", &word[..i], &word[i + 1..]);
            let lst = adj_list.entry(key).or_insert(vec![]);
            lst.push(word);
        }
    }

    adj_list
}

#[cfg(test)]
mod tests {

    use super::ladder_length;

    #[test] 
    pub fn test_ladder_length() {

        assert_eq!(ladder_length("hit", "cog", &["hot","dot","dog","lot","log","cog"]), 5);
    }

    #[test] 
    pub fn test_ladder_length_without_end_word() {
        assert_eq!(ladder_length("hit", "cog", &["hot","dot","dog","lot","log"]), 0);
    }

    #[test]
    pub fn test_ladder_length_mama() {

        assert_eq!(ladder_length("MAMA", "SIRI", &["SAMA", "SIMA", "SIRA", "SIRI", "MISA", "DISA"]), 5);
    }
}