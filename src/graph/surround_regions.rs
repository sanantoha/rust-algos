
// O(w * h) time | O(w * h) space
pub fn solve(board: &mut [&mut [char]]) {
    if board.is_empty() {
        return;
    }

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if (row == 0 || col == 0 || row == board.len() - 1 || col == board[row].len() - 1)
                && board[row][col] == 'O' {
                dfs(board, row, col);
            }
        }
    }

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] == '*' {
                board[row][col] = 'O';
            } else if board[row][col] == 'O' {
                board[row][col] = 'X';
            }
        }
    }
}

fn dfs(board: &mut [&mut [char]], start_row: usize, start_col: usize) {
    let mut stack = vec![(start_row, start_col)];

    while let Some((row, col)) = stack.pop() {
        if board[row][col] != 'O' {
            continue;
        }
        board[row][col] = '*';

        stack.extend(get_neighbors(board, row, col));
    }
}

fn get_neighbors(board: &mut [&mut [char]], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < board.len() - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < board[row].len() - 1 {
        neighbors.push((row, col + 1));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use crate::graph::surround_regions::solve;

    #[test]
    fn test_solve() {
        let board: &mut [&mut [char]] = &mut[
            &mut ['X','X','X','X'],
            &mut ['X','O','O','X'],
            &mut ['X','X','O','X'],
            &mut ['X','O','X','X'],
        ];

        let exp: &[&[char]] = &[
            &['X','X','X','X'],
            &['X','X','X','X'],
            &['X','X','X','X'],
            &['X','O','X','X'],
        ];

        solve(board);

        assert_eq!(board, exp);
    }

    #[test]
    fn test_solve_case1() {
        let board: &mut [&mut [char]] = &mut[
            &mut ['O', 'O', 'O'],
            &mut ['O', 'O', 'O'],
            &mut ['O', 'O', 'O'],
        ];

        let exp: &[&[char]] = &[
            &['O', 'O', 'O'],
            &['O', 'O', 'O'],
            &['O', 'O', 'O'],
        ];

        solve(board);

        assert_eq!(board, exp);
    }

    #[test]
    fn test_solve_case2() {
        let board: &mut [&mut [char]] = &mut [
            &mut ['X','O','X','O','X','O'],
            &mut ['O','X','O','X','O','X'],
            &mut ['X','O','X','O','X','O'],
            &mut ['O','X','O','X','O','X'],
        ];

        let exp: &[&[char]] = &[
            &['X','O','X','O','X','O'],
            &['O','X','X','X','X','X'],
            &['X','X','X','X','X','O'],
            &['O','X','O','X','O','X'],
        ];

        solve(board);

        assert_eq!(board, exp);
    }
}