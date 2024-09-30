
// O(w * h) time | O(w * h) space
pub fn largest_increasing_path_in_matrix(matrix: &[&[i32]]) -> i32 {
    if matrix.is_empty() {
        return 0;
    }

    let mut cache = vec![vec![0; matrix[0].len()]; matrix.len()];

    let mut ans = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            ans = ans.max(dfs(matrix, i, j, &mut cache));
        }
    }

    ans
}

fn dfs(matrix: &[&[i32]], row: usize, col: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
    if cache[row][col] != 0 {
        return cache[row][col];
    }

    let val = matrix[row][col];
    for (nrow, ncol) in get_neighbors(matrix, row, col) {

        if matrix[nrow][ncol] > val {
            cache[row][col] = cache[row][col].max(dfs(matrix, nrow, ncol, cache));
        }
    }
    cache[row][col] += 1;
    cache[row][col]
}

fn get_neighbors(matrix: &[&[i32]], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < matrix.len() - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < matrix[0].len() - 1 {
        neighbors.push((row, col + 1));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use crate::graph::largest_increasing_path_in_matrix::largest_increasing_path_in_matrix;

    #[test]
    fn test_largest_increasing_path_in_matrix() {
        let matrix: &[&[i32]] = &[
            &[9,9,4],
            &[6,6,8],
            &[2,1,1],
        ];

        assert_eq!(largest_increasing_path_in_matrix(matrix), 4);
    }

    #[test]
    fn test_largest_increasing_path_in_matrix_case1() {
        let matrix: &[&[i32]] = &[
            &[3,4,5],
            &[3,2,6],
            &[2,2,1],
        ];

        assert_eq!(largest_increasing_path_in_matrix(matrix), 4);
    }
}