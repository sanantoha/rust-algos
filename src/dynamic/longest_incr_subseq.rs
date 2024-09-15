
// O(n ^ 2) time | O(n) space
pub fn lis0(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut lis = vec![1; arr.len()];

    let mut max_val = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] < arr[i] && lis[i] < lis[j] + 1 {
                lis[i] = lis[j] + 1;
            }
        }
        if max_val < lis[i] {
            max_val = lis[i];
        }
    }

    max_val
}

// O(n * log(n)) time | O(n) space
pub fn lis(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut res = vec![];
    res.push(arr[0]);

    for i in 1..arr.len() {
        let prev = res[res.len() - 1];
        if prev < arr[i] {
            res.push(arr[i]);
        } else {
            let j = match res.binary_search(&arr[i]) {
                Ok(i) => i,
                Err(i) => i,
            };
            res[j] = arr[i];
        }
    }

    res.len() as i32
}

// O(n ^ 2) time | O(n) space
pub fn lis_list0(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut lis = vec![1; arr.len()];
    let mut prev = vec![None; arr.len()];

    let mut max_val = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] < arr[i] && lis[i] < lis[j] + 1 {
                lis[i] = lis[j] + 1;
                prev[i] = Some(j);
            }
        }

        if lis[max_val] < lis[i] {
            max_val = i;
        }
    }

    build_list(arr, &prev, Some(max_val))
}

fn build_list(arr: &[i32], prev: &[Option<usize>], max_val: Option<usize>) -> Vec<i32> {
    let mut res = vec![];

    let mut idx = max_val;

    while let Some(i) = idx.take() {
        res.push(arr[i]);
        idx = prev[i];
    }

    res.reverse();
    res
}

// O(n * log(n)) time | O(n) space
fn lis_list(arr: &[i32]) -> Vec<i32> {
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

    build_list(arr, &prev, indices[length])
}

fn binary_search(arr: &[i32], indices: &[Option<usize>], mut l: usize, mut r: usize, target: i32) -> usize {
    while l <= r {
        let mid = (l + r) >> 1;
        if let Some(idx) = indices[mid] {
            if target > arr[idx] {
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
    use crate::dynamic::longest_incr_subseq::{lis0, lis, lis_list0, lis_list};

    const ARR0: &[i32] = &[1, 2, 3, 6, -100, -90, -80, -70, -60, 7, 8, 9, 10, -50, -40];

    const ARR1: &[i32] = &[10, 22, 9, 33, 21, 50, 41, 60, 80];

    const ARR2: &[i32] = &[4,10,4,3,8,9];

    const ARR3: &[i32] = &[10,9,2,5,3,7,101,18];

    const ARR4: &[i32] = &[1, -10, 20, 30, 2, 3, 4, 5];

    #[test]
    fn test_lis0_case0() {
        assert_eq!(lis0(ARR0), 9);
    }

    #[test]
    fn test_lis0_case1() {
        assert_eq!(lis0(ARR1), 6);
    }

    #[test]
    fn test_lis0_case2() {
        assert_eq!(lis0(ARR2), 3);
    }

    #[test]
    fn test_lis0_case3() {
        assert_eq!(lis0(ARR3), 4);
    }

    #[test]
    fn test_lis0_case4() {
        assert_eq!(lis0(ARR4), 5);
    }

    #[test]
    fn test_lis_case0() {
        assert_eq!(lis(ARR0), 9);
    }

    #[test]
    fn test_lis_case1() {
        assert_eq!(lis(ARR1), 6);
    }

    #[test]
    fn test_lis_case2() {
        assert_eq!(lis(ARR2), 3);
    }

    #[test]
    fn test_lis_case3() {
        assert_eq!(lis(ARR3), 4);
    }

    #[test]
    fn test_lis_case4() {
        assert_eq!(lis(ARR4), 5);
    }

    #[test]
    fn test_lis_list0_case0() {
        assert_eq!(lis_list0(ARR0), vec![-100, -90, -80, -70, -60, 7, 8, 9, 10]);
    }

    #[test]
    fn test_lis_list0_case1() {
        assert_eq!(lis_list0(ARR1), vec![10, 22, 33, 50, 60, 80]);
    }

    #[test]
    fn test_lis_list0_case2() {
        assert_eq!(lis_list0(ARR2), vec![4, 8, 9]);
    }

    #[test]
    fn test_lis_list0_case3() {
        assert_eq!(lis_list0(ARR3), vec![2, 5, 7, 101]);
    }

    #[test]
    fn test_lis_list0_case4() {
        assert_eq!(lis_list0(ARR4), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_lis_list_case0() {
        assert_eq!(lis_list(ARR0), vec![-100, -90, -80, -70, -60, 7, 8, 9, 10]);
    }

    #[test]
    fn test_lis_list_case1() {
        assert_eq!(lis_list(ARR1), vec![10, 22, 33, 41, 60, 80]);
    }

    #[test]
    fn test_lis_list_case2() {
        assert_eq!(lis_list(ARR2), vec![3, 8, 9]);
    }

    #[test]
    fn test_lis_list_case3() {
        assert_eq!(lis_list(ARR3), vec![2, 3, 7, 18]);
    }

    #[test]
    fn test_lis_list_case4() {
        assert_eq!(lis_list(ARR4), vec![-10, 2, 3, 4, 5]);
    }
}