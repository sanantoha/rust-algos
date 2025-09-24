
pub fn max_sum_increasing_subsequence(arr: &[i32]) -> Vec<Vec<i32>> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::max_sum_increasing_subsequence;

    #[test]
    fn test_max_sum_increasing_subsequence() {
        let input: &[i32] = &[10, 70, 20, 30, 50, 11, 30];

        let res = max_sum_increasing_subsequence(input);
        println!("{:?}", res);
        assert_eq!(res, vec![vec![110], vec![10, 20, 30, 50]]);
    }
}