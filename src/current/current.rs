

pub fn subarray_sort(arr: &[i32]) -> Option<(usize, usize)> {
    None
}

#[cfg(test)]
mod tests {

    use super::subarray_sort;

    #[test]
    fn it_subarray_sort() {
        let arr = &[1, 2, 4, 7, 10, 11, 7, 12, 6, 7, 16, 18, 19];

        assert_eq!(subarray_sort(arr), Some((3, 9)));
    }

    #[test]
    fn it_subarray_sort_case() {
        let arr = &[1, 2, 4, 7, 10, 11, 7, 12, 7, 7, 16, 18, 19];

        assert_eq!(subarray_sort(arr), Some((4, 9)));
    }

    #[test]
    fn it_subarray_sort_case1() {
        let arr = &[1, 2];

        assert_eq!(subarray_sort(arr), None);
    }
}