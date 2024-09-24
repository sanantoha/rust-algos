
#[derive(Debug)]
struct MinHeap {
    heap: Vec<i32>
}

impl MinHeap {
    
    fn new(arr: &[i32]) -> Self {
        let heap = MinHeap::build_heap(arr);

        MinHeap {
            heap
        }
    }   

    // O(log(n)) time | O(1) space
    fn insert(&mut self, value: i32) {
        self.heap.push(value);
        let idx = self.heap.len();
        MinHeap::sift_up(idx - 1, &mut self.heap);
    }

    // O(log(n)) time | O(1) space
    fn peek(&self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }
        Some(self.heap[0])
    }

    // O(log(n)) time | O(1) space
    fn remove(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }
        let l = self.heap.len();
        self.heap.swap(0, l - 1);
        let v = self.heap.remove(l - 1);
        let l = self.heap.len();
        MinHeap::sift_down(0, l - 1, &mut self.heap);
        Some(v)
    }

    // O(n) time | O(1) space
    fn build_heap(arr: &[i32]) -> Vec<i32> {
        let mut heap = arr.to_vec();
        let size = heap.len();
        for i in (0..arr.len() / 2).rev() {
            MinHeap::sift_down(i, size - 1, &mut heap);
        }

        heap
    }

    // O(log(n)) time | O(1) space
    fn sift_down(curr_idx: usize, end_idx: usize, heap: &mut Vec<i32>) {

        let mut idx = curr_idx;

        while idx <= end_idx {
            let l = idx * 2 + 1;
            let r = idx * 2 + 2;

            let mut min_idx = idx;
            if l <= end_idx && heap[min_idx] > heap[l] {
                min_idx = l;
            }
            if r <= end_idx && heap[min_idx] > heap[r] {
                min_idx = r;
            }
            if idx != min_idx {
                heap.swap(min_idx, idx);
                idx = min_idx;
            } else {
                break;
            }
        }
    }

    // O(log(n)) time | O(1) space
    fn sift_up(curr_idx: usize, heap: &mut Vec<i32>) {
        let mut idx = curr_idx;

        while let Some(i) = idx.checked_sub(1) {
            let parent_id = i / 2;

            if heap[parent_id] > heap[idx] {
                heap.swap(parent_id, idx);
                idx = parent_id;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinHeap;


    #[test]
    fn it_min_heap() {
        let arr = &[0, 1, 2];

        let mut heap = MinHeap::new(arr);
        heap.insert(3);
        heap.insert(4);
        heap.insert(5);

        println!("{:?}", heap.heap);
        assert_eq!(heap.heap, vec![0, 1, 2, 3, 4, 5]);

        println!("{:?}", heap.peek());
        assert_eq!(heap.peek(), Some(0));
                
        assert_eq!(heap.remove(), Some(0));
        println!("{:?}", heap.heap);
        assert_eq!(heap.heap, vec![1, 3, 2, 5, 4]);

        assert_eq!(heap.remove(), Some(1));
        assert_eq!(heap.remove(), Some(2));

        heap.insert(4);
        println!("{:?}", heap.heap);
        assert_eq!(heap.heap, vec![3, 4, 4, 5]);
    }
}