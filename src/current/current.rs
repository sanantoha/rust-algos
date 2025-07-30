
pub fn unique_path_iii(grid: &mut Vec<Vec<i32>>) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::unique_path_iii;

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