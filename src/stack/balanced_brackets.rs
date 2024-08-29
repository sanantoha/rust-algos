use std::collections::{HashMap, HashSet, VecDeque};

// O(n) time | O(n) space
pub fn balanced_brackets(str: &str) -> bool {

    let mut stack = VecDeque::new();
    let mut map = HashMap::new();
    map.insert('{', '}');
    map.insert('[', ']');
    map.insert('(', ')');
    let set: HashSet<char> = map.values().cloned().collect();

    for c in str.chars() {
        if map.contains_key(&c) {
            if let Some(close_c) = map.get(&c) {
                stack.push_back(close_c);
            }            
        } else if set.contains(&c) {
            if stack.is_empty() {
                return false;
            }
            if let Some(&stack_c) = stack.pop_back() {
                if stack_c != c {
                    return false;
                }
            }
        }
        
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {

    use super::balanced_brackets;

    #[test]
    fn it_balanced_brackets() {
        assert!(balanced_brackets("([])(){}(())()()"));
    }

    #[test]
    fn it_im_balanced_brackets() {
        assert!(!balanced_brackets("}{"));
    }
}