
pub fn exists(board: &mut Vec<Vec<char>>, word: &str) -> bool {
    false
}


#[cfg(test)]
mod tests {
    use super::exists;

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