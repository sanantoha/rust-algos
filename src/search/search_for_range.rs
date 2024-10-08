
// O(log(n)) time | O(1) space
pub fn search_range(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    if arr.is_empty() {
        return None;
    }
    let left = left_binary_search(arr, target);
    left.and_then(|l| right_binary_search(arr, l, target))
}

fn right_binary_search(arr: &[i32], left: usize, target: i32) -> Option<(usize, usize)> {
    let mut l = left;
    let mut r = arr.len() - 1;
    while l <= r {
        let mid = l + (r - l) / 2;
        if target < arr[mid] {
            if mid == 0 {
                break;
            }
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    println!("{}", r);
    Some((left, r))
}

fn left_binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let mid = l + (r - l) / 2;
        if target <= arr[mid] {
            if mid == 0 {
                break;
            }
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    if arr[l] == target {
        Some(l)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::search::search_for_range::search_range;

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