
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

    fn insert(&mut self, value: i32) {

    }

    fn peek(&self) -> Option<i32> {
        None
    }

    fn remove(&mut self) -> Option<i32> {
        None
    }

    fn build_heap(arr: &[i32]) -> Vec<i32> {
        vec![]
    }

    fn sift_down(curr_idx: usize, end_idx: usize, heap: &mut Vec<i32>) {

    }

    fn sift_up(curr_idx: usize, heap: &mut Vec<i32>) {

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