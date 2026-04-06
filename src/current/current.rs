
pub fn subarray_sum(nums: &[i32], k: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::subarray_sum;

    #[test]
    fn it_subarray_sum() {
        assert_eq!(subarray_sum(&[1, 1, 1], 2), 2);
    }

    #[test]
    fn it_subarray_sum_case2() {
        assert_eq!(subarray_sum(&[1, 2, 3], 3), 2);
    }
}