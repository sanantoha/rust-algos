
// O(n) time | O(n) space
pub fn product(arr: &[i32]) -> Vec<i32> {

    let mut res = vec![0; arr.len()];

    let mut left_product = 1;

    for i in 0..arr.len() {
        res[i] = left_product;
        left_product *= arr[i];
    }

    let mut right_product = 1;

    for i in (0..arr.len()).rev() {
        res[i] *= right_product;
        right_product *= arr[i];
    }

    res
}

#[cfg(test)]
mod tests {

    use super::product;

    #[test]
    fn it_product() {
        assert_eq!(product(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn it_product_if_one_zero() {
        assert_eq!(product(&[1, 2, 0, 4]), vec![0, 0, 8, 0]);
    }

    #[test]
    fn it_product_if_two_zero() {
        assert_eq!(product(&[0, 2, 0, 4]), vec![0, 0, 0, 0]);
    }
}