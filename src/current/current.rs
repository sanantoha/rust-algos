
pub fn longest_common_subsequence(str1: &str, str2: &str) -> Vec<char> {
    vec![]
}


pub fn longest_common_subsequence1(str1: &str, str2: &str) -> Vec<char> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::{longest_common_subsequence, longest_common_subsequence1};

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(longest_common_subsequence("ZXVVYZW", "XKYKZPW"), vec!['X', 'Y', 'Z', 'W']);
    }

    #[test]
    fn test_longest_common_subsequence1() {
        assert_eq!(longest_common_subsequence1("ZXVVYZW", "XKYKZPW"), vec!['X', 'Y', 'Z', 'W']);
    }
}