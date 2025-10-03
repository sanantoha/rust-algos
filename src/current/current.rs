
pub fn search_range(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    None
}

#[cfg(test)]
mod tests {
    use super::search_range;

    #[test]
    fn test_search_for_range() {
        let arr: &[i32] = &[5,7,7,8,8,8,8,8,8,8,10];

        assert_eq!(search_range(arr, 8), Some((3, 9)));
    }

    #[test]
    fn test_search_for_range_case1() {
        let arr: &[i32] = &[5,7,7,8,8,8,8,8,8,8,10];

        assert_eq!(search_range(arr, 6), None);
    }

    #[test]
    fn test_search_for_range_case2() {
        let arr: &[i32] = &[1];

        assert_eq!(search_range(arr, 1), Some((0,0)));
    }

    #[test]
    fn test_search_for_range_case3() {
        let arr: &[i32] = &[];

        assert_eq!(search_range(arr, 0), None);
    }
}