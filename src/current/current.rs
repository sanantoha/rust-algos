
pub fn maximum_subarray(arr: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::maximum_subarray;

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