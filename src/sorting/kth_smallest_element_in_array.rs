
// O(n) time | O(1) space
pub fn kth_smallest_element_in_array(arr: &mut [usize], k: usize) -> Option<usize> {
    helper(arr, 0, arr.len() - 1, k)
}

fn helper(arr: &mut [usize], l: usize, r: usize, k: usize) -> Option<usize> {
    if l > r {
        return None;
    }

    let p = partition(arr, l, r);
    if p + 1 == k {
        return Some(arr[p]);
    } else if p + 1 < k {
        return helper(arr, p + 1, r, k);
    }
    if let Some(p_minus_one) = p.checked_sub(1) {
        return helper(arr, l, p_minus_one, k);
    } else {
        return None;
    }
}

fn partition(arr: &mut [usize], l: usize, r: usize) -> usize {
    let mut j = l;
    for i in l..r {
        if arr[i] <= arr[r] {
            arr.swap(i, j);
            j += 1;
        }
    }
    arr.swap(r, j);
    j
}

#[cfg(test)]
mod tests {
    use crate::sorting::kth_smallest_element_in_array::kth_smallest_element_in_array;

    #[test]
    fn test_kth_smallest_element_in_array() {
        let mut arr: Vec<usize> = vec![8, 2, 1, 6, 9, 3, 45, 6, 7, 13];
        let mut arr_copy: Vec<usize> = arr.clone();
        arr_copy.sort();

        for i in 0..arr.len() {
            let exp = arr_copy[i];
            assert_eq!(kth_smallest_element_in_array(&mut arr, i + 1), Some(exp));
        }
    }

    #[test]
    fn test_kth_smallest_element_in_array_case1() {
        let mut arr: Vec<usize> = vec![1, 2, 3];
        assert_eq!(kth_smallest_element_in_array(&mut arr, 0), None);
    }

    #[test]
    fn test_kth_smallest_element_in_array_case2() {
        let mut arr: Vec<usize> = vec![1, 2, 3];
        assert_eq!(kth_smallest_element_in_array(&mut arr, 11), None);
    }
}