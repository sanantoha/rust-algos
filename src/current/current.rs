
pub fn three_sum(arr: &[i32], target: i32) -> Vec<[i32; 3]> {
    vec![]
}

pub fn three_sum1(arr: &[i32], target: i32) -> Vec<[i32; 3]> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::three_sum;
    use super::three_sum1;

    #[test]
    fn test_three_sum() {
        assert_eq!(three_sum(&vec![12, 3, 1, 2, -6, 5, -8, 6], 0), vec![[3, 5, -8], [1, -6, 5], [2, -8, 6]]);
    }

    #[test]
    fn test_three_sum1() {
        assert_eq!(three_sum1(&vec![12, 3, 1, 2, -6, 5, -8, 6], 0), vec![[-8, 2, 6], [-8, 3, 5], [-6, 1, 5]]);
    }
}