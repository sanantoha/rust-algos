
pub fn unique_paths(m: usize, n: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::unique_paths;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn test_unique_paths_case1() {
        assert_eq!(unique_paths(3, 7), 28);
    }
}