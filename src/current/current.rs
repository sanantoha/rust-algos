
pub fn largest_island(matrix: &[&[i32]]) -> i32 {
    -1
}

pub fn largest_island1(matrix: &mut [&mut [i32]]) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::{largest_island, largest_island1};

    const MATRIX: &[&[i32]] = &[
        &[1, 0, 1, 0, 0],
        &[0, 0, 1, 1, 0],
        &[0, 1, 1, 1, 1],
        &[1, 0, 1, 0, 0],
    ];

    const EXP: i32 = 8;

    #[test]
    fn test_largest_island() {

        assert_eq!(largest_island(MATRIX), EXP);
    }

    #[test]
    fn test_largest_island1() {
        // let mut matrix: Vec<Vec<i32>> = MATRIX.to_vec().into_iter().map(|v| v.to_vec()).collect();
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 0, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 0],
        ];

        let mut mut_matrix: Vec<&mut [i32]> = matrix.iter_mut().map(|row| row.as_mut_slice()).collect();

        assert_eq!(largest_island1(&mut mut_matrix), EXP);
    }
}