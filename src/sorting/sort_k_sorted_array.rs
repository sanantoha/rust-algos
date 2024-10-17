use std::cmp::Reverse;
use std::collections::BinaryHeap;

// O(n * log(k)) time | O(k) space
pub fn sort_k_sorted_array(arr: &mut [i32], k: i32) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mut heap = BinaryHeap::new();
    for i in 0..((k + 1) as usize).min(arr.len()) {
        heap.push(Reverse(arr[i]));
    }

    let mut idx = 0;

    for i in ((k + 1) as usize)..arr.len() {
        if let Some(Reverse(x)) = heap.pop() {
            arr[idx] = x;
            idx += 1;
        }

        heap.push(Reverse(arr[i]));
    }

    while let Some(Reverse(x)) = heap.pop() {
        arr[idx] = x;
        idx += 1;
    }

    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use crate::sorting::sort_k_sorted_array::sort_k_sorted_array;

    #[test]
    fn basic() {
        let mut arr: Vec<i32> = vec![3, 2, 1, 5, 4, 7, 6, 5];

        assert_eq!(sort_k_sorted_array(&mut arr, 3), vec![1, 2, 3, 4, 5, 5, 6, 7]);
    }
}