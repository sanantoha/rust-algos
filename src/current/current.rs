
pub fn combination_sum(arr: &[i32], target: i32) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::combination_sum;

    #[test]
    fn it_combination_sum() {
        let arr = &[2, 3, 5, 7];

        assert_eq!(combination_sum(arr, 7), vec![vec![2, 2, 3], vec![2, 5], vec![7]]);
    }
}