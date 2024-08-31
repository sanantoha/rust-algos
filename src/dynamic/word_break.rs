use std::collections::HashSet;

// O(n ^ 3) time | O(n) space
pub fn word_break(str: &str, word_dict: &[&str]) -> bool {
    if str.is_empty() || word_dict.is_empty() {
        return false;
    }

    let mut dp = vec![false; str.len() + 1];
    dp[0] = true;

    let word_set: HashSet<&str> = HashSet::from_iter(word_dict.iter().cloned());

    for i in 1..=str.len() {
        for j in 0..i {
            if dp[j] && word_set.contains(&str[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    
    dp[str.len()]
}


#[cfg(test)]
mod tests {

    use super::word_break;

    #[test]
    fn it_word_break() {

        assert!(word_break("leetcode", &["leet", "code"]));
    }

    #[test]
    fn it_word_break1() {

        assert!(word_break("applepenapple", &["apple", "pen"]));
    }

    #[test]
    fn it_word_break_impossible() {

        assert!(!word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]));
    }
}