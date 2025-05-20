
pub fn search_matrix(matrix: &[&[i32]], target: i32) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::search_matrix;

    #[test]
    fn test_search_matrix() {
        let matrix: &[&[i32]] = &[
            &[1, 4, 7, 11, 15],
            &[2, 5, 8, 12, 19],
            &[3, 6, 9, 16, 22],
            &[10,13,14, 17, 24],
            &[18,21,23, 26, 30],
        ];

        assert!(search_matrix(matrix, 21));
    }

    #[test]
    fn test_search_matrix_case1() {
        let matrix: &[&[i32]] = &[
            &[1, 4],
            &[2, 5],
        ];

        assert!(search_matrix(matrix, 5));
    }
}