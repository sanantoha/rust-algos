
pub fn update_matrix(_matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec![]
}



#[cfg(test)]
mod tests {

    use super::update_matrix;

    #[test]
    pub fn test_update_matrix_case1() {
        let input = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        let exp = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];

        let res = update_matrix(input);

        assert_eq!(res, exp);
    }

    #[test]
    pub fn test_update_matrix_case2() {
        let input = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![1, 1, 1],
        ];
        let exp = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![1, 2, 1],
        ];

        let res = update_matrix(input);

        assert_eq!(res, exp);
    }
}