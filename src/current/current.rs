
pub fn binary_search(arr: &[i32], target: i32) -> i32 {
    -1
}


#[cfg(test)]
mod tests {

    use super::binary_search;

    const ARR: &[i32] = &[10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(ARR, 80), 7);
    }

    #[test]
    fn test_binary_search_missing_target() {
        assert_eq!(binary_search(ARR, 81), -9);
    }
}