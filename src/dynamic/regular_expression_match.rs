use std::collections::HashSet;

// O(s * p) time | O(s * p) space
pub fn is_match(s: &str, p: &str) -> bool {
    let mut memo = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp(s, p, 0, 0, &mut memo)
}

fn dp(s: &str, p: &str, i: usize, j: usize, memo: &mut Vec<Vec<bool>>) -> bool {
    if !memo[i][j] {
        let  ans: bool;
        if j == p.len() {
            ans = i == s.len();
        } else {
            let set: HashSet<char> = [s.chars().nth(i).unwrap_or_default(), '.'].iter().copied().collect();
            let first_match = i < s.len() && set.contains(&p.chars().nth(j).unwrap_or_default());

            if j + 1 < p.len() && p.chars().nth(j + 1).unwrap_or_default() == '*' {
                ans = dp(s, p, i, j + 2, memo) || first_match && dp(s, p, i + 1, j, memo);
            } else {
                ans = first_match && dp(s, p, i + 1, j + 1, memo);
            }
        }
        memo[i][j] = ans;
    }
    memo[i][j]
}

// O(s * p) time | O(s * p) space
pub fn is_match_iter(s: &str, p: &str) -> bool {

    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[s.len()][p.len()] = true;

    for i in (0..=s.len()).rev() {
        for j in (0..p.len()).rev() {
            let set: HashSet<char> = [s.chars().nth(i).unwrap_or_default(), '.'].iter().copied().collect();
            let first_match = i < s.len() && set.contains(&p.chars().nth(j).unwrap_or_default());

            if j + 1 < p.len() && p.chars().nth(j + 1).unwrap_or_default() == '*' {
                dp[i][j] = dp[i][j + 2] || first_match && dp[i + 1][j];
            } else {
                dp[i][j] = first_match && dp[i + 1][j + 1];
            }
        }
    }


    dp[0][0]
}

#[cfg(test)]
mod tests {
    use crate::dynamic::regular_expression_match::{is_match, is_match_iter};

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