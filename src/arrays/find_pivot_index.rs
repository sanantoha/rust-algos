
// O(n) time | O(1) space
pub fn find_pivot_index(arr: &[i32]) -> i32 {

    let mut left_sum = 0;
    let mut right_sum = arr.iter().sum();

    for (i, v) in arr.iter().enumerate() {
        right_sum -= v;

        if left_sum == right_sum {
            return i as i32;
        }

        left_sum += v;
    }

    -1
}

#[cfg(test)]
mod tests {

    use super::find_pivot_index;

    #[test]
    fn test_find_pivot_index() {

        assert_eq!(find_pivot_index(&[1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(find_pivot_index(&[1, 2, 3]), -1);
        assert_eq!(find_pivot_index(&[2, 1, -1]), 0);
    }
}
