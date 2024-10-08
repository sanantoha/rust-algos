
// O(w + h) time | O(1) space
pub fn search_matrix(matrix: &[&[i32]], target: i32) -> bool {
    if matrix.is_empty() {
        return false;
    }

    let mut col = matrix[0].len() - 1;
    let mut row = 0;

    while row < matrix.len() {
        if matrix[row][col] == target {
            return true;
        } else if matrix[row][col] < target {
            row += 1;
        } else {
            if col == 0 {
                return false;
            }
            col -= 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::search::search_2_d_matrix::search_matrix;

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