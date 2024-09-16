
// O(n) time | O(1) space
pub fn maximum_subarray(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut max_subarr = 0;
    let mut sum = 0;

    for num in arr {
        sum = (*num).max(*num + sum);
        max_subarr = max_subarr.max(sum);
    }

    max_subarr
}

#[cfg(test)]
mod tests {
    use crate::dynamic::maximum_subarray::maximum_subarray;

    #[test]
    fn test_maximum_subarray() {

        assert_eq!(maximum_subarray(&[3, 5, -9, 1, 3, -2, 3, 4, 7, 2, -9, 6, 3, 1, -5, 4]), 19);
    }

    #[test]
    fn test_maximum_subarray1() {

        assert_eq!(maximum_subarray(&[-2,1,-3,4,-1,2,1,-5,4]), 6);
    }

    #[test]
    fn test_maximum_subarray2() {

        assert_eq!(maximum_subarray(&[3, 4, -6, 7, 8, -18, 100]), 100);
    }
}