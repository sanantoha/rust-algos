use std::collections::VecDeque;

// O(w * h) time | O(w * h) space
pub fn minimum_passes_of_matrix(matrix: &mut [&mut [i32]]) -> i32 {

    let mut queue = get_all_positives(matrix);
    let mut count = 0;

    while !queue.is_empty() {
        let mut size = queue.len();

        count += 1;

        while size > 0 {
            size -= 1;
            if let Some((row, col)) = queue.pop_front() {
                for (nrow, ncol) in get_neighbors(matrix, row, col) {
                    if matrix[nrow][ncol] < 0 {
                        matrix[nrow][ncol] *= -1;
                        queue.push_back((nrow, ncol));
                    }
                }
            }
        }
    }


    if negative_cell(matrix) {
        -1
    } else {
        count - 1
    }
}

fn negative_cell(matrix: &mut [&mut [i32]]) -> bool {
    for row in 0..matrix.len() {
        for cell in 0..matrix[row].len() {
            if matrix[row][cell] < 0 {
                return true;
            }
        }
    }

    false
}

fn get_neighbors(matrix: &mut [&mut [i32]], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row + 1 < matrix.len() {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col + 1 < matrix[row].len() {
        neighbors.push((row, col + 1));
    }

    neighbors
}

fn get_all_positives(matrix: &mut [&mut [i32]]) -> VecDeque<(usize, usize)> {
    let mut queue = VecDeque::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] > 0 {
                queue.push_back((i, j));
            }
        }
    }

    queue
}


#[cfg(test)]
mod tests {
    use crate::graph::minimum_passes_of_matrix::minimum_passes_of_matrix;

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