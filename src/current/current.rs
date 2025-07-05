

fn number_of_islands(grid: &[&[i32]]) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::number_of_islands;

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