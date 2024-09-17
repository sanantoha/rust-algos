
// O(n ^ 2) time | O(n) space
pub fn max_sum_increasing_subsequence(arr: &[i32]) -> Vec<Vec<i32>> {
    if arr.is_empty() {
        return vec![];
    }

    let mut msi = arr.to_vec();
    let mut prev = vec![None; arr.len()];

    let mut max_id = 0;

    for i in 1..msi.len() {
        for j in 0..i {
            if arr[j] < arr[i] && msi[i] < msi[j] + arr[i] {
                msi[i] = msi[j] + arr[i];
                prev[i] = Some(j);
            }
        }
        if msi[max_id] < msi[i] {
            max_id = i;
        }
    }

    vec![vec![msi[max_id]], build_seq(arr, &prev, Some(max_id))]
}

fn build_seq(arr: &[i32], prev: &[Option<usize>], max_id: Option<usize>) -> Vec<i32> {
    let mut res = vec![];

    let mut idx = max_id;

    while let Some(i) = idx.take() {
        res.push(arr[i]);
        idx = prev[i];
    }

    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use crate::dynamic::max_sum_increasing_subsequence::max_sum_increasing_subsequence;

    #[test]
    fn test_max_sum_increasing_subsequence() {
        let input: &[i32] = &[10, 70, 20, 30, 50, 11, 30];

        let res = max_sum_increasing_subsequence(input);
        println!("{:?}", res);
        assert_eq!(res, vec![vec![110], vec![10, 20, 30, 50]]);
    }
}