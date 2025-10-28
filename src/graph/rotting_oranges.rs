use std::collections::VecDeque;

// O(w * h) time | O(w * h) space
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() {
        return 0
    }

    let mut count = 0;

    let mut m_grid = grid;

    let grid_slice: Vec<&[i32]> = m_grid.iter().map(|i| i.as_slice()).collect();
    let mut queue = find_all_rotten(&grid_slice);

    while !queue.is_empty() {
        let mut size = queue.len();

        count += 1;

        while size > 0 {
            size -= 1;

            if let Some((row, col)) = queue.pop_front() {
                let grid_slice: Vec<&[i32]> = m_grid.iter().map(|i| i.as_slice()).collect();
                for (n_row, n_col) in get_neighbors(&grid_slice, row, col) {
                    if m_grid[n_row][n_col] != 1 {
                        continue;
                    }
                    m_grid[n_row][n_col] *= -1;
                    queue.push_back((n_row, n_col));
                }
            }
        }
    }


    let grid_slice: Vec<&[i32]> = m_grid.iter().map(|i| i.as_slice()).collect();
    if is_all_rotten(&grid_slice) {
        count - 1
    } else if at_least_one_fresh(&grid_slice) {
        -1
    } else {
        0
    }
}

fn at_least_one_fresh(grid: &[&[i32]]) -> bool {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 {
                return true;
            }
        }
    }
    false
}

fn is_all_rotten(grid: &[&[i32]]) -> bool {
    let mut at_least_one_rotten = false;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 {
                return false;
            } else if grid[row][col] == 2 {
                at_least_one_rotten = true;
            }
        }
    }
    at_least_one_rotten
}

fn get_neighbors(grid: &[&[i32]], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if row > 0 {
        res.push((row - 1, col));
    }
    if col > 0 {
        res.push((row, col - 1));
    }
    if row + 1 < grid.len() {
        res.push((row + 1, col));
    }
    if col + 1 < grid[row].len() {
        res.push((row, col + 1));
    }
    res
}

fn find_all_rotten(grid: &[&[i32]]) -> VecDeque<(usize, usize)> {
    let mut queue = VecDeque::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 2 { // rotten
                queue.push_back((row, col));
            }
        }
    }

    queue
}

#[cfg(test)]
mod tests {

    use super::oranges_rotting;

    #[test]
    pub fn test_case1() {
        let grid = vec![
            vec![2,1,1],
            vec![1,1,0],
            vec![0,1,1]
        ];

        let res = oranges_rotting(grid);
        println!("test case1: {}", res);
        assert_eq!(res, 4);
    }

    #[test]
    pub fn test_case2() {
        let grid = vec![
            vec![2,1,1],
            vec![0,1,1],
            vec![1,0,1]
        ];

        let res = oranges_rotting(grid);
        println!("test case2: {}", res);
        assert_eq!(res, -1);
    }

    #[test]
    pub fn test_case3() {
        let grid = vec![
            vec![0,2]
        ];

        let res = oranges_rotting(grid);
        println!("test case3: {}", res);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_case4() {
        let grid = vec![
            vec![0]
        ];

        let res = oranges_rotting(grid);
        println!("test case4: {}", res);
        assert_eq!(res, 0);
    }

}

