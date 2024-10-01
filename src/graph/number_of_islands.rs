
// O(w * h) time | O(w * h) space
pub fn number_of_islands(grid: &[&[i32]]) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let mut count = 0;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 && !visited[row][col] {
                dfs(grid, &mut visited, row, col);
                count += 1;
            }
        }
    }

    count
}

fn dfs(grid: &[&[i32]], visited: &mut [Vec<bool>], start_row: usize, start_col: usize) {

    let mut stack = vec![(start_row, start_col)];

    while let Some((row, col)) = stack.pop() {
        if visited[row][col] || grid[row][col] == 0 {
            continue;
        }
        visited[row][col] = true;

        stack.extend(get_neighbors(grid, row, col));
    }
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

#[cfg(test)]
mod test {
    use crate::graph::number_of_islands::number_of_islands;

    #[test]
    fn test_number_of_islands() {
        let grid: &[&[i32]] = &[
            &[0, 0, 0, 0, 1],
            &[1, 1, 0, 0, 0],
            &[1, 1, 0, 1, 1],
            &[0, 0, 0, 1, 1],
        ];

        assert_eq!(number_of_islands(grid), 3);
    }
}