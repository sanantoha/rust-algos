
// O(n ^ 2) time | O(1) space
pub fn count_substrings(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut count = 0;

    for i in 0..s.chars().count() {
        count += palindrome_count(s, i, i);
        count += palindrome_count(s, i, i + 1);
    }

    count
}

fn palindrome_count(s: &str, mut l: usize, mut r: usize) -> usize {
    let mut count = 0;

    while r < s.chars().count() {
        if s.chars().nth(l) != s.chars().nth(r) {
            break;
        }
        count += 1;
        if let Some(ll) = l.checked_sub(1) {
            l = ll;
        } else {
            break;
        }
        r += 1;
    }
    count
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