

// O(n) time | O(1) space
pub fn subarray_sort(arr: &[i32]) -> Option<(usize, usize)> {

    let mut min_out_of_order = i32::MAX;
    let mut max_out_of_order = i32::MIN;

    for i in 0..arr.len() {
        if is_out_of_order(arr, i) {
            min_out_of_order = min_out_of_order.min(arr[i]);
            max_out_of_order = max_out_of_order.max(arr[i]);
        }
    }

    if min_out_of_order == i32::MAX {
        return None;
    }

    let mut l = 0;
    while arr[l] <= min_out_of_order {        
        l += 1;
    }

    let mut r = arr.len() - 1;
    while arr[r] >= max_out_of_order {
        r -= 1;
    }
    
    Some((l, r))
}

fn is_out_of_order(arr: &[i32], idx: usize) -> bool {
    if arr.len() <= 1 {
        return false;
    }

    if idx == 0 {
        arr[idx] > arr[idx + 1] 
    } else if idx == arr.len() - 1 {
        arr[idx - 1] > arr[idx]
    } else {
        arr[idx - 1] > arr[idx] || arr[idx] > arr[idx + 1]
    }
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