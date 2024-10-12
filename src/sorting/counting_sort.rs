
pub fn counting_sort(arr: &mut [i32]) {

    let mut min_val = i32::MAX;
    let mut max_val = i32::MIN;

    for v in arr.iter() {
        min_val = min_val.min(*v);
        max_val = max_val.max(*v);
    }

    let mut cnt = vec![0; (max_val - min_val + 1) as usize];

    for v in arr.iter() {
        cnt[*v as usize - min_val as usize] += 1;
    }

    let mut idx = 0;
    for i in 0..cnt.len() {
        for _ in 0..cnt[i] {
            arr[idx] = i as i32 + min_val;
            idx += 1;
        }
    }
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