use std::collections::VecDeque;


// O(w * h) time | O(w * h) space
pub fn river_sizes(matrix: &[&[i32]]) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }


    let mut res = vec![];

    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];    

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 && !visited[i][j] {
                res.push(dfs_size(matrix, &mut visited, i, j));
            }
        }
    }


    res
}

fn dfs_size(matrix: &[&[i32]], visited: &mut [Vec<bool>], start_row: usize, start_col: usize) -> i32 {
    let mut size = 0;

    let mut stack = VecDeque::new();
    stack.push_back((start_row, start_col));

    while !stack.is_empty() {
        if let Some((row, col)) = stack.pop_back() {

            if matrix[row][col] != 1 || visited[row][col] {
                continue;
            }
            visited[row][col] = true;
            size += 1;

            for (nrow, ncol) in get_neighbors(matrix, row, col) {
                stack.push_back((nrow, ncol));
            }
        }        
    }


    size
}

fn get_neighbors(matrix: &[&[i32]], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];

    if row > 0 {
        res.push((row - 1, col));
    }
    if col > 0 {
        res.push((row, col - 1));
    }
    if row + 1 < matrix.len() {
        res.push((row + 1, col));
    }
    if col + 1 < matrix[row].len() {
        res.push((row, col + 1));
    }

    res
}

#[cfg(test)]
mod tests {

    use super::river_sizes;

    #[test]
    pub fn test_river_sizes() {

        let matrix: &[&[i32]] = &[
            &[1, 0, 0, 1, 0],
            &[1, 0, 1, 0, 1],
            &[0, 0, 1, 0, 1],
            &[1, 0, 1, 1, 0],
            &[1, 0, 1, 0, 0],
        ];

        let res = river_sizes(matrix);
        println!("{:?}", res);

        assert_eq!(res, vec![2, 1, 5, 2, 2])
    }
}