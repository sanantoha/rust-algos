use std::collections::VecDeque;

// O(w * h) time | O(w * h) space
pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut matrix_mv = create_matrix_replace_one_on_max_val(matrix);

    let mut queue = find_all_zeros(&matrix_mv);

    while let Some((row, col, level)) = queue.pop_front() {

        for (nrow, ncol) in get_neighborns(&matrix_mv, row, col) {
            if matrix_mv[nrow][ncol] == i32::MAX {
                queue.push_back((nrow, ncol, level + 1));
            }
            matrix_mv[nrow][ncol] = matrix_mv[nrow][ncol].min(level + 1);
        }
    }

    println!("{:?}", matrix_mv);
    matrix_mv
}

fn get_neighborns(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<(usize, usize)> {
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

fn find_all_zeros(matrix: &Vec<Vec<i32>>) -> VecDeque<(usize, usize, i32)> {
    let mut res = VecDeque::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 0 {
                res.push_back((row, col, 0));
            }
        }
    }
    res
}

fn create_matrix_replace_one_on_max_val(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = matrix.to_vec();
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 1 {
                res[row][col] = i32::MAX;
            }
        }
    }
    res
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