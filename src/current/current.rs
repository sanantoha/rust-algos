pub fn word_break(str: &str, word_dict: &[&str]) -> bool {
    false
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