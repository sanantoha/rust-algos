
pub fn minimum_passes_of_matrix(matrix: &mut [&mut [i32]]) -> i32 {
    -1
}


#[cfg(test)]
mod tests {
    use super::minimum_passes_of_matrix;

    #[test]
    fn test_minimum_passes_of_matrix() {
        let mut matrix = vec![
            vec![0, -1, -3, 2, 0],
            vec![1, -2, -5, -1, -3],
            vec![3, 0, 0, -4, -1]
        ];

        let mut mut_matrix: Vec<&mut [i32]> = matrix.iter_mut().map(|v| v.as_mut_slice()).collect();

        let res = minimum_passes_of_matrix(&mut mut_matrix);
        println!("{}", res);

        assert_eq!(res, 3);
    }
}