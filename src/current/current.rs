
pub fn kth_smallest_element_in_array(arr: &mut [usize], k: usize) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::kth_smallest_element_in_array;

    #[test]
    fn test_kth_smallest_element_in_array() {
        let mut arr: Vec<usize> = vec![8, 2, 1, 6, 9, 3, 45, 6, 7, 13];
        let mut arr_copy: Vec<usize> = arr.clone();
        arr_copy.sort();

        for i in 0..arr.len() {
            let exp = arr_copy[i];
            assert_eq!(kth_smallest_element_in_array(&mut arr, i + 1), Some(exp));
        }
    }

    #[test]
    fn test_kth_smallest_element_in_array_case1() {
        let mut arr: Vec<usize> = vec![1, 2, 3];
        assert_eq!(kth_smallest_element_in_array(&mut arr, 0), None);
    }

    #[test]
    fn test_kth_smallest_element_in_array_case2() {
        let mut arr: Vec<usize> = vec![1, 2, 3];
        assert_eq!(kth_smallest_element_in_array(&mut arr, 11), None);
    }
}