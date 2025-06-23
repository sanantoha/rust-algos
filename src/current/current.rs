
pub fn longest_increasing_path_in_matrix(matrix: &[&[i32]]) -> i32 {
    0
}



#[cfg(test)]
mod tests {
    use super::longest_increasing_path_in_matrix;

    #[test]
    fn test_largest_increasing_path_in_matrix() {
        let matrix: &[&[i32]] = &[
            &[9,9,4],
            &[6,6,8],
            &[2,1,1],
        ];

        assert_eq!(longest_increasing_path_in_matrix(matrix), 4);
    }

    #[test]
    fn test_largest_increasing_path_in_matrix_case1() {
        let matrix: &[&[i32]] = &[
            &[3,4,5],
            &[3,2,6],
            &[2,2,1],
        ];

        assert_eq!(longest_increasing_path_in_matrix(matrix), 4);
    }
}