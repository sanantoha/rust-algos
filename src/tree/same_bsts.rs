
// O(n ^ 2) time | O(n ^ 2) space
pub fn same_bsts(array_one: &[i32], array_two: &[i32]) -> bool {
    if array_one.is_empty() && array_two.is_empty() {
        return true;
    }
    if array_one.len() != array_two.len() {
        return false;
    }
    if array_one[0] != array_two[0] {
        return false;
    }

    let (less_one, greater_one) = partition(array_one);
    let (less_two, greater_two) = partition(array_two);

    same_bsts(&less_one, &less_two) && same_bsts(&greater_one, &greater_two)
}

fn partition(array: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut less = vec![];
    let mut greater = vec![];

    for i in 1..array.len() {
        if array[i] < array[0] {
            less.push(array[i]);
        } else {
            greater.push(array[i]);
        }
    }
    (less, greater)
}

// O(n ^ 2) time | O(d) space
pub fn same_bsts1(array_one: &[i32], array_two: &[i32]) -> bool {
    if array_one.is_empty() && array_two.is_empty() {
        return true;
    }
    if array_one.len() != array_two.len() {
        return false;
    }

    helper(array_one, array_two, Some(0), Some(0), i32::MIN, i32::MAX)
}

fn helper(array_one: &[i32], array_two: &[i32], idx_one_opt: Option<usize>, idx_two_opt: Option<usize>, min_val: i32, max_val: i32) -> bool {
    if let (Some(idx_one), Some(idx_two)) = (idx_one_opt, idx_two_opt) {
        if array_one[idx_one] != array_two[idx_two] {
            return false;
        }

        let less_idx_one = get_less_idx(array_one, idx_one, min_val);
        let less_idx_two = get_less_idx(array_two, idx_two, min_val);
        let greater_idx_one = get_greater_idx(array_one, idx_one, max_val);
        let greater_idx_two = get_greater_idx(array_two, idx_two, max_val);
        let val = array_one[idx_one];
        helper(array_one, array_two, less_idx_one, less_idx_two, min_val, val) &&
            helper(array_one, array_two, greater_idx_one, greater_idx_two, val, max_val)
    } else {
        idx_one_opt.is_none() == idx_two_opt.is_none()
    }

}

fn get_less_idx(array: &[i32], idx: usize, min_val: i32) -> Option<usize> {
    for i in (idx + 1)..array.len() {
        if array[i] < array[idx] && array[i] >= min_val {
            return Some(i);
        }
    }
    None
}

fn get_greater_idx(array: &[i32], idx: usize, max_val: i32) -> Option<usize> {
    for i in (idx + 1)..array.len() {
        if array[i] >= array[idx] && array[i] < max_val {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::tree::same_bsts::{same_bsts, same_bsts1};

    const ARR_ONE: &[i32] = &[10, 15, 8, 12, 94, 81, 5, 2, 11];
    const ARR_TWO: &[i32] = &[10, 8, 5, 15, 2, 12, 11, 94, 81];

    #[test]
    fn test_same_bsts() {
        assert!(same_bsts(ARR_ONE, ARR_TWO));
    }

    #[test]
    fn test_same_bsts1() {
        assert!(same_bsts1(ARR_ONE, ARR_TWO));
    }
}