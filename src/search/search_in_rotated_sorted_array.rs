
// O(log(n)) time | O(1) space
pub fn search(arr: &[i32], target: i32) -> Result<usize, usize> {

    let smallest_idx = find_smallest_idx(arr);

    let mut l = 0;
    let mut r = arr.len() - 1;

    if target >= arr[smallest_idx] && target <= arr[r]{
        l = smallest_idx;
    } else {
        r = smallest_idx - 1;
    }

    while l <= r {
        let mid = l + (r - l) / 2;

        if target < arr[mid] {
            r = mid - 1;
        } else if target > arr[mid] {
            l = mid + 1;
        } else {
            return Ok(mid);
        }
    }

    if arr[l] == target {
        Ok(l)
    } else {
        Err(l)
    }
}

fn find_smallest_idx(arr: &[i32]) -> usize {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let mid = l + (r - l) / 2;
        if arr[mid] <= arr[r] {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    l
}

// O(log(n)) time | O(1) space
pub fn search1(arr: &[i32], target: i32) -> Result<usize, usize> {

    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let mid = l + (r - l) / 2;

        if target == arr[mid] {
            return Ok(mid);
        }

        if arr[mid] <= arr[r] {
            if target > arr[mid] && target <= arr[r] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        } else {
            if target >= arr[l] && target < arr[mid] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
    }

    Err(l)
}

#[cfg(test)]
mod tests {
    use super::{search, search1};

    const ARR: &[i32] = &[40, 50, 60, 70, 80, 90, 0, 10, 20, 30, 31, 32, 33, 34, 35];
    const TARGET: i32 = 90;

    #[test]
    fn test_search() {
        assert_eq!(search(ARR, TARGET), Ok(5));
    }

    #[test]
    fn test_search_case1() {
        assert_eq!(search(ARR, 11), Err(8));
    }

    #[test]
    fn test_search1() {
        assert_eq!(search1(ARR, TARGET), Ok(5));
    }

    #[test]
    fn test_search1_case1() {
        assert_eq!(search1(ARR, 11), Err(8));
    }
}