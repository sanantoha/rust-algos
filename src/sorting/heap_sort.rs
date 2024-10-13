
// O(n * log(n)) time | O(1) space
pub fn heap_sort(arr: &mut [i32]) {
    let mut size = arr.len() - 1;
    build_heap(size, arr);

    while size > 0 {
        arr.swap(0, size);
        size -= 1;
        sift_down(0, size, arr);
    }
}

fn build_heap(size: usize, arr: &mut [i32]) {
    for i in (0..(size / 2)).rev() {
        sift_down(i, size, arr);
    }
}

fn sift_down(idx: usize, end_idx: usize, arr: &mut [i32]) {
    let mut curr_idx = idx;

    while curr_idx <= end_idx {
        let l = left(curr_idx);
        let r = right(curr_idx);
        let mut max_idx = curr_idx;

        if l <= end_idx && arr[max_idx] < arr[l] {
            max_idx = l;
        }
        if r <= end_idx && arr[max_idx] < arr[r] {
            max_idx = r;
        }
        if curr_idx != max_idx {
            arr.swap(max_idx, curr_idx);
            curr_idx = max_idx;
        } else {
            break;
        }
    }
}

fn left(i: usize) -> usize {
    i * 2 + 1
}

fn right(i: usize) -> usize {
    i * 2 + 2
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::heap_sort;

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];

        heap_sort(&mut arr);

        assert_eq!(arr, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_heap_sort_case1() {
        let mut arr = vec![0; 30];

        let mut rand = thread_rng();

        for i in 0..arr.len() {
            arr[i] = rand.gen_range(0..100)
        }

        heap_sort(&mut arr);

        println!("{:?}", arr);

        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }
}