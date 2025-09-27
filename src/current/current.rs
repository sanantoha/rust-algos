
pub fn a_star_algorithm(start_row: usize, start_col: usize, end_row: usize, end_col: usize, graph: &[&[i32]]) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::a_star_algorithm;

    #[test]
    fn test_a_star_algorithm() {
        let start_row = 0;
        let start_col = 1;
        let end_row = 4;
        let end_col = 3;

        let graph: &[&[i32]] = &[
            &[0, 0, 0, 0, 0],
            &[0, 1, 1, 1, 0],
            &[0, 0, 0, 0, 0],
            &[1, 0, 1, 1, 1],
            &[0, 0, 0, 0, 0],
        ];

        let expected = vec![
            vec![0, 1],
            vec![0, 0],
            vec![1, 0],
            vec![2, 0],
            vec![2, 1],
            vec![3, 1],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
        ];

        let res = a_star_algorithm(start_row, start_col, end_row, end_col, graph);
        println!("{:?}", res);
        assert_eq!(res, expected);
    }
}