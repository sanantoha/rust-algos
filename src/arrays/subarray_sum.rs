use std::collections::HashMap;

// O(n) time | O(n) space
pub fn subarray_sum(nums: &[i32], k: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut sum = 0;
    let mut count = 0;

    let mut map = HashMap::new();
    map.insert(0, 1);

    for v in nums.iter() {
        sum += v;

        if let Some(cnt) = map.get(&(sum - k)) {
            count += cnt;
        }

        let cnt = map.entry(sum).or_insert(0);
        *cnt += 1;
    }

    count
}

#[cfg(test)]
mod tests {

    use super::subarray_sum;

    #[test]
    fn it_subarray_sum() {
        assert_eq!(subarray_sum(&[1, 1, 1], 2), 2);
    }

    #[test]
    fn it_subarray_sum_case2() {
        assert_eq!(subarray_sum(&[1, 2, 3], 3), 2);
    }
}