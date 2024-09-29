use std::collections::HashSet;

// O(w ^ 2 * h ^ 2) time | O(w * h) space
pub fn largest_island(matrix: &[&[i32]]) -> i32 {
    if matrix.is_empty() {
        return 0;
    }

    let mut largest = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] != 1 {
                continue;
            }

            largest = largest.max(dfs_size(matrix, row, col));
        }
    }

    largest
}

fn dfs_size(matrix: &[&[i32]], start_row: usize, start_col: usize) -> i32 {
    let mut size = 1;

    let mut stack = get_neighbors(matrix, start_row, start_col);
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    while let Some((row, col)) = stack.pop() {
        if visited[row][col] || matrix[row][col] != 0 {
            continue;
        }
        visited[row][col] = true;
        size += 1;

        stack.extend(get_neighbors(matrix, row, col));
    }

    size
}

fn get_neighbors(matrix: &[&[i32]], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if row + 1 < matrix.len() {
        neighbors.push((row + 1, col));
    }
    if col + 1 < matrix[row].len() {
        neighbors.push((row, col + 1));
    }
    neighbors
}


// O(w * h) time | O(w * h) space
pub fn largest_island1(matrix: &mut [&mut [i32]]) -> i32 {

    let mut island_num = 2;
    let mut islands_sizes = vec![];

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 0 {
                let size = dfs_size1(matrix, row, col, island_num);
                islands_sizes.push(size);
                island_num += 1;
            }
        }
    }

    let mut largest = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] != 1 {
                continue;
            }

            let mut islands = HashSet::new();
            let slices: &[&[i32]] = &matrix.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
            for (row, col) in get_neighbors(slices, row, col) {
                if matrix[row][col] == 1 {
                    continue;
                }
                islands.insert(matrix[row][col]);
            }

            let mut size = 1;
            for island in islands {
                size += islands_sizes[(island - 2) as usize];
            }
            largest = largest.max(size);
        }
    }
    largest
}

fn dfs_size1(matrix: &mut [&mut [i32]], start_row: usize, start_col: usize, island_num: usize) -> i32 {

    let mut stack = vec![(start_row, start_col)];
    let mut size = 0;

    while let Some((row, col)) = stack.pop() {
        if matrix[row][col] != 0 {
            continue;
        }
        matrix[row][col] = island_num as i32;
        size += 1;

        let slices: &[&[i32]] = &matrix.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
        stack.extend(get_neighbors(slices, row, col));
    }

    size
}


#[cfg(test)]
mod tests {
    use crate::graph::largest_island::{largest_island, largest_island1};

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