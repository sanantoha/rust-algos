
// O(w * h) time | O(w * h) space
pub fn generate_matrix(n: usize) -> Vec<Vec<i32>> {

    let mut res = vec![vec![0; n]; n];

    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = n - 1;
    let mut end_col = n - 1;

    let mut idx = 1;

    while start_row <= end_row && start_col <= end_col {
        for i in start_col..=end_col {
            res[start_row][i] = idx;
            idx += 1;
        }
        start_row += 1;

        for i in start_row..=end_row {
            res[i][end_col] = idx;
            idx += 1;
        }
        end_col = end_col.saturating_sub(1);

        if start_row <= end_row && start_col <= end_col {
            for i in (start_col..=end_col).rev() {
                res[end_row][i] = idx;
                idx += 1;
            }
            end_row = end_row.saturating_sub(1);       

            for i in (start_row..=end_row).rev() {
                res[i][start_col] = idx;
                idx += 1;
            }
            start_col += 1;
        }
    }

    res
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