
pub fn solve(board: &mut [&mut [char]]) {

}

#[cfg(test)]
mod tests {
    use super::solve;

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