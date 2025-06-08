
pub fn product(arr: &[i32]) -> Vec<i32> {
    vec![]
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