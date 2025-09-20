
pub fn count_substrings(src: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::count_substrings;

    #[test]
    fn test_count_substrings() {
        assert_eq!(count_substrings("abc"), 3);
    }

    #[test]
    fn test_count_substrings_case1() {
        assert_eq!(count_substrings("aaa"), 6);
    }

    #[test]
    fn test_count_substrings_case2() {
        assert_eq!(count_substrings("aabbbaa"), 14);
    }

    #[test]
    fn test_count_substrings_case3() {
        assert_eq!(count_substrings("aaab"), 7);
    }
}