
// O(m * n) time | O(m * n) space
pub fn unique_paths(m: usize, n: usize) -> usize {

    let mut dp = vec![vec![1; n]; m];

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use crate::dynamic::unique_paths::unique_paths;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn test_unique_paths_case1() {
        assert_eq!(unique_paths(3, 7), 28);
    }
}