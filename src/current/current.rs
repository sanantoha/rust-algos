

pub fn counting_sort(arr: &mut [i32]) {

}


#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];

        counting_sort(&mut arr);

        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_counting_sort_case1() {
        let mut arr = vec![0; 30];

        let mut rand = thread_rng();

        for i in 0..arr.len() {
            arr[i] = rand.gen_range(0..100)
        }

        counting_sort(&mut arr);

        println!("{:?}", arr);

        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }
}