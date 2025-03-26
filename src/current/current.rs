
pub fn generate_matrix(n: usize) -> Vec<Vec<i32>> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::generate_matrix;

    #[test]
    fn test_generate_matrix_for_3() {
        assert_eq!(generate_matrix(3), vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]])
    }

    #[test]
    fn test_generate_matrix_for_1() {
        assert_eq!(generate_matrix(1), vec![vec![1]])
    }

    #[test]
    fn test_generate_matrix_for_4() {
        assert_eq!(generate_matrix(4), vec![vec![1, 2, 3, 4], vec![12, 13, 14, 5], vec![11, 16, 15, 6], vec![10, 9, 8, 7]])
    }
}