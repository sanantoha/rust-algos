
// O(n ^ 2) time | O(n) space
pub fn lds(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut lds = vec![1; arr.len()];

    let mut max_val = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] > arr[i] && lds[i] < lds[j] + 1 {
                lds[i] = lds[j] + 1;
            }
        }
        if max_val < lds[i] {
            max_val = lds[i];
        }
    }

    max_val
}

// O(n * log(n)) time | O(n) space
pub fn lds1(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut res = vec![arr[0]];

    for i in 1..arr.len() {
        let prev = res[res.len() - 1];
        if prev > arr[i] {
            res.push(arr[i]);
        } else {
            let j = res.binary_search_by(|p| p.cmp(&arr[i]).reverse()).unwrap_or_else(|i| i);

            res[j] = arr[i];
        }
    }

    res.len() as i32
}

// O(n ^ 2) time | O(n) space
pub fn lds_list(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut lds = vec![1; arr.len()];
    let mut prev = vec![None; arr.len()];

    let mut max_idx = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] > arr[i] && lds[i] < lds[j] + 1 {
                lds[i] = lds[j] + 1;
                prev[i] = Some(j);
            }
        }
        if lds[max_idx] < lds[i] {
            max_idx = i;
        }
    }

    build_seq(arr, &prev, Some(max_idx))
}

fn build_seq(arr: &[i32], prev: &[Option<usize>], max_idx: Option<usize>) -> Vec<i32> {
    let mut idx = max_idx;

    let mut res = vec![];

    while let Some(i) = idx.take() {
        res.push(arr[i]);
        idx = prev[i];
    }

    res.reverse();
    res
}

// O(n * log(n)) time | O(n) space
pub fn lds_list1(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut indices = vec![None; arr.len() + 1];
    let mut prev = vec![None; arr.len()];

    let mut length = 0;

    for i in 0..arr.len() {
        let max_len = binary_search(arr, &indices, 1, length, arr[i]);
        indices[max_len] = Some(i);
        prev[i] = indices[max_len - 1];
        length = length.max(max_len);
    }

    build_seq(arr, &prev, indices[length])
}

fn binary_search(arr: &[i32], indices: &[Option<usize>], mut l: usize, mut r: usize, target: i32) -> usize {
    while l <= r {
        let mid = l + (r - l) / 2;
        if let Some(idx) = indices[mid] {
            if target < arr[idx] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    l
}

#[cfg(test)]
mod tests {
    use crate::dynamic::longest_decr_subseq::{lds, lds1, lds_list, lds_list1};

    const ARR0: &[i32] = &[5,6,7,6,5,4,3,10,14,12];

    const ARR1: &[i32] = &[100, 10, 9, 8, 7, 6, 5, 90, 80, 70, 60, 50, 40, 30, 20];

    #[test]
    fn test_lds_case0() {
        assert_eq!(lds(ARR0), 5);
    }

    #[test]
    fn test_lds_case1() {
        assert_eq!(lds(ARR1), 9);
    }

    #[test]
    fn test_lds1_case0() {
        assert_eq!(lds1(ARR0), 5);
    }

    #[test]
    fn test_lds1_case1() {
        assert_eq!(lds1(ARR1), 9);
    }

    #[test]
    fn test_lds_list_case0() {
        assert_eq!(lds_list(ARR0), vec![7, 6, 5, 4, 3]);
    }

    #[test]
    fn test_lds_list_case1() {
        assert_eq!(lds_list(ARR1), vec![100, 90, 80, 70, 60, 50, 40, 30, 20]);
    }

    #[test]
    fn test_lds_list1_case0() {
        assert_eq!(lds_list1(ARR0), vec![7, 6, 5, 4, 3]);
    }

    #[test]
    fn test_lds_list1_case1() {
        assert_eq!(lds_list1(ARR1), vec![100, 90, 80, 70, 60, 50, 40, 30, 20]);
    }
}