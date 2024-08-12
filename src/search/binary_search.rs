
// O(log(n)) time | O(1) space
pub fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = arr.len();

    while l <= r {
        let mid = l + (r - l) / 2;
        if target < arr[mid] {
            r = mid - 1;
        } else if target > arr[mid] {
            l = mid + 1;
        } else {
            return mid as i32;
        }
    }
    return -((l + 1) as i32)
}

#[test]
fn test_binary_search() {
    let arr = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    assert_eq!(binary_search(&arr, 80), 7);
    assert_eq!(binary_search(&arr, 81), -9);
}