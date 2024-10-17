use std::collections::VecDeque;

// O(n) time | O(n) space
pub fn next_greater_element(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut res = vec![-1; arr.len()];

    let mut stack = VecDeque::new();

    for i in 0..(2 * arr.len()) {
        let idx = i % arr.len();

        while let Some(&x) = stack.back() {
            if arr[x] < arr[idx] {
                res[x] = arr[idx];
                stack.pop_back();
            } else {
                break;
            }
        }

        stack.push_back(idx);
    }

    res
}

// O(n) time | O(n) space
pub fn next_greater_element1(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut res = vec![-1; arr.len()];

    let mut stack = VecDeque::new();

    for i in (0..2 * arr.len()).rev() {
        let idx = i % arr.len();

        while let Some(&x) = stack.back() {
            if arr[idx] < x {
                res[idx] = x;
                break;
            }
            stack.pop_back();
        }

        stack.push_back(arr[idx]);
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    const ARR: &[i32] = &[2, 5, -3, -4, 6, 7, 2];

    #[test]
    fn test_next_greater_element() {
        assert_eq!(next_greater_element(ARR), vec![5, 6, 6, 6, 7, -1, 5])
    }

    #[test]
    fn test_next_greater_element1() {
        assert_eq!(next_greater_element1(ARR), vec![5, 6, 6, 6, 7, -1, 5])
    }
}