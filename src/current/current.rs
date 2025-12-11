
pub fn is_match(s: &str, p: &str) -> bool {
    false
}

pub fn is_match_iter(s: &str, p: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{is_match, is_match_iter};

    #[test]
    fn test_is_match() {
        assert!(!is_match("aa", "a"));
    }

    #[test]
    fn test_is_match_case1() {
        assert!(is_match("aa", "a*"));
    }

    #[test]
    fn test_is_match_case2() {
        assert!(is_match("abcde", ".*"));
    }

    #[test]
    fn test_is_match_case3() {
        assert!(is_match("abcde", ".*de"));
    }

    #[test]
    fn test_is_match_case4() {
        assert!(!is_match("abcde", ".*dk"));
    }

    #[test]
    fn test_is_match_iter() {
        assert!(!is_match_iter("aa", "a"));
    }

    #[test]
    fn test_is_match_iter_case1() {
        assert!(is_match_iter("aa", "a*"));
    }

    #[test]
    fn test_is_match_iter_case2() {
        assert!(is_match_iter("abcde", ".*"));
    }

    #[test]
    fn test_is_match_iter_case3() {
        assert!(is_match_iter("abcde", ".*de"));
    }

    #[test]
    fn test_is_match_iter_case4() {
        assert!(!is_match_iter("abcde", ".*dk"));
    }
}