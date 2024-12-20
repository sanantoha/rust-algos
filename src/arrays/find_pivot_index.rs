
// O(n) time | O(1) space
pub fn find_pivot_index(arr: &[i32]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut left_sum = 0;
    let mut right_sum = arr.iter().sum();

    for i in 0..arr.len() {
        right_sum -= arr[i];

        if left_sum == right_sum {
            return Some(i);
        }

        left_sum += arr[i];
    }
    None
}

#[cfg(test)]
mod tests {

    use super::find_pivot_index;

    #[test]
    fn test_find_pivot_index() {

        assert_eq!(find_pivot_index(&[1, 7, 3, 6, 5, 6]), Some(3));
        assert_eq!(find_pivot_index(&[1, 2, 3]), None);
        assert_eq!(find_pivot_index(&[2, 1, -1]), Some(0));
    }
}
