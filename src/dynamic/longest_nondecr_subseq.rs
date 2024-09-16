
// O(n ^ 2) time | O(n) space
pub fn lnds(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut lnds = vec![1; arr.len()];

    let mut max_val = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] <= arr[i] && lnds[i] < lnds[j] + 1 {
                lnds[i] = lnds[j] + 1;
            }
        }

        if max_val < lnds[i] {
            max_val = lnds[i];
        }
    }

    max_val
}

// O(n * log(n)) time | O(n) space
pub fn lnds1(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut res = vec![arr[0]];

    for i in 1..arr.len() {
        let prev = res[res.len() - 1];
        if prev <= arr[i] {
            res.push(arr[i]);
        } else {
            let j = binary_search(&res, arr[i]).unwrap_or_else(|i| i);
            res[j] = arr[i];
        }
    }

    res.len() as i32
}

fn binary_search(arr: &[i32], target: i32) -> Result<usize, usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let mid = l + (r - l) / 2;
        if target < arr[mid] {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    if arr[l] == target {
        Ok(l)
    } else {
        Err(l)
    }
}

// O(n ^ 2) time | O(n) space
pub fn lnds_list(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut lnds = vec![1; arr.len()];
    let mut prev = vec![None; arr.len()];

    let mut max_idx = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] <= arr[i] && lnds[i] < lnds[j] + 1 {
                lnds[i] = lnds[j] + 1;
                prev[i] = Some(j);
            }
        }
        if lnds[max_idx] < lnds[i] {
            max_idx = i;
        }
    }

    build_seq(arr, &prev, Some(max_idx))
}

fn build_seq(arr: &[i32], prev: &[Option<usize>], max_idx: Option<usize>) -> Vec<i32> {
    let mut res = vec![];

    let mut idx = max_idx;

    while let Some(i) = idx.take() {
        res.push(arr[i]);
        idx = prev[i];
    }

    res.reverse();
    res
}

// O(n * log(n)) time | O(n) space
pub fn lnds_list1(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut indices = vec![None; arr.len() + 1];
    let mut prev = vec![None; arr.len()];

    let mut length = 0;

    for i in 0..arr.len() {
        let max_len = binary_search_on_indices(arr, &indices, 1, length, arr[i]);
        indices[max_len] = Some(i);
        prev[i] = indices[max_len - 1];
        length = length.max(max_len);
    }

    build_seq(arr, &prev, indices[length])
}

fn binary_search_on_indices(arr: &[i32], indices: &[Option<usize>], mut l: usize, mut r: usize, target: i32) -> usize {
    while l <= r {
        let mid = l + (r - l) / 2;
        if let Some(idx) = indices[mid] {
            if target < arr[idx] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
    }
    l
}

#[cfg(test)]
mod tests {

    use super::{lnds, lnds1, lnds_list, lnds_list1};

    const ARR: &[i32] = &[-2, -1, 2, 3, 4, 5, 2, 2, 2, 2, 3];
    #[test]
    fn test_lnds() {
        assert_eq!(lnds(ARR), 8);
    }

    #[test]
    fn test_lnds1() {
        assert_eq!(lnds1(ARR), 8);
    }

    #[test]
    fn test_lnds_list() {
        assert_eq!(lnds_list(ARR), vec![-2, -1, 2, 2, 2, 2, 2, 3]);
    }

    #[test]
    fn test_lnds_list1() {
        assert_eq!(lnds_list1(ARR), vec![-2, -1, 2, 2, 2, 2, 2, 3]);
    }
}