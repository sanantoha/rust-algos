
// O(log(n)) time | O(1) space
pub fn binary_search(arr: &[i32], target: i32) -> i32 {
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
    -((l + 1) as i32)
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