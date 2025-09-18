
// O(n * 3 ^ l) time | O(l) space - where l is length of the word
pub fn exists(board: &mut Vec<Vec<char>>, word: &str) -> bool {

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if let Some(c) = word.chars().nth(0) {
                if c == board[row][col] && dfs(board, row, col, 0, word) {
                    return true;
                }
            }
        }
    }
    false
}

fn dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize, idx: usize, word: &str) -> bool {
    if word.len() == idx {
        return true;
    }

    if row >= board.len() || col >= board[0].len() {
        return false;
    }
    if let Some(c) = word.chars().nth(idx) {
        if c != board[row][col] {
            return false;
        }
    }

    let tmp = board[row][col];
    board[row][col] = '*';

    let row_minus_one_branch = if let Some(row_minus_one) = row.checked_sub(1) {
        dfs(board, row_minus_one, col, idx + 1, word)
    } else {
        false
    };
    let col_minus_one_branch = if let Some(col_minus_one) = col.checked_sub(1) {
        dfs(board, row, col_minus_one, idx + 1, word)
    } else {
        false
    };

    let is_found = row_minus_one_branch || col_minus_one_branch ||
        dfs(board, row + 1, col, idx + 1, word) || dfs(board, row, col + 1, idx + 1, word);

    board[row][col] = tmp;

    is_found
}

#[cfg(test)]
mod tests {
    use crate::backtracking::word_search::exists;

    #[test]
    fn it_exists() {
        let mut board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];

        assert!(exists(&mut board, "ABCCE"));
    }

    #[test]
    fn it_exists_case1() {
        let mut board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','E','S'],
            vec!['A','D','E','E']
        ];

        assert!(exists(&mut board, "ABCESEEEFS"));
    }

    #[test]
    fn it_exists_case2() {
        let mut board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','E','S'],
            vec!['A','D','E','E']
        ];

        assert!(exists(&mut board, "ABCEFSADEESE"));
    }

    #[test]
    fn it_exists_case3() {
        let mut board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','E','S'],
            vec!['A','D','E','E']
        ];

        assert!(!exists(&mut board, "ABCEV"));
    }
}