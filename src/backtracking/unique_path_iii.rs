
// O(3 ^ n) time | O(n) space
pub fn unique_path_iii(grid: &mut Vec<Vec<i32>>) -> i32 {

    let mut start_row = 0;
    let mut start_col = 0;
    let mut zeros = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 0 {
                zeros += 1;
            } else if grid[row][col] == 1 {
                start_row = row;
                start_col = col;
            }
        }
    }

    paths(grid, start_row, start_col, zeros)
}

fn paths(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, zeros: i32) -> i32 {
    if row >= grid.len() || col >= grid[row].len() || grid[row][col] == -1 {
        return 0;
    }

    if grid[row][col] == 2 {
        return if zeros == -1 { 1 } else { 0 }
    }

    grid[row][col] = -1;
    let nzero = zeros - 1;

    let left = if let Some(row_minus_one) = row.checked_sub(1) {
        paths(grid, row_minus_one, col, nzero)
    } else {
        0
    };
    let right = if let Some(col_minus_one) = col.checked_sub(1) {
        paths(grid, row, col_minus_one, nzero)
    } else {
        0
    };
    let total_count = left + right + paths(grid, row + 1, col, nzero) + paths(grid, row, col + 1, nzero);

    grid[row][col] = 0;

    total_count
}

#[cfg(test)]
mod tests {
    use crate::backtracking::unique_path_iii::unique_path_iii;

    #[test]
    fn it_unique_path_iii() {
        let mut grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 2, -1]
        ];

        assert_eq!(unique_path_iii(&mut grid), 2);
    }
}