
// O(N ^ (M / T + 1)) time | O(T / M) space
// where N is the number of candidates, M is the smallest candidate among all the given integers,
// and T is the target value.
// Thus the time complexity is exponential and this is expected because the algorithm is
// recursive backtracking.
pub fn combination_sum(arr: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut ans = vec![];
    backtrack(arr, target, 0, &mut ans, &mut res);
    res
}

fn backtrack(arr: &[i32], target: i32, idx: usize, ans: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if target < 0 {
        return;
    }
    if target == 0 {
        res.push(ans.clone());
        return;
    }

    for i in idx..arr.len() {
        ans.push(arr[i]);
        backtrack(arr, target - arr[i], i, ans, res);
        ans.remove(ans.len() - 1);
    }
}

#[cfg(test)]
mod tests {

    use super::combination_sum;

    #[test]
    fn it_combination_sum() {
        let arr = &[2, 3, 5, 7];

        assert_eq!(combination_sum(arr, 7), vec![vec![2, 2, 3], vec![2, 5], vec![7]]);
    }
}