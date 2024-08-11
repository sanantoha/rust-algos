
// O(n) time | O(1) space
pub fn find_pivot_index(arr: Vec<i32>) -> i32 {

    let mut left_sum = 0;
    let mut right_sum = arr.iter().sum();

    for i in 0..arr.len() {
        right_sum -= arr[i];

        if left_sum == right_sum {
            return i as i32;
        }

        left_sum += arr[i];
    }

    return -1
}

#[test]
fn test_find_pivot_index() {

    assert_eq!(find_pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(find_pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(find_pivot_index(vec![2, 1, -1]), 0);
}