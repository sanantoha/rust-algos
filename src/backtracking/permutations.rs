
// O(n * n!) time | O(n * n!) space
pub fn permute(arr: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    backtrack(arr, 0, &mut res);
    res
}

fn backtrack(arr: &mut Vec<i32>, idx: usize, res: &mut Vec<Vec<i32>>) {
    if arr.len() == idx {
        res.push(arr.to_vec());
        return;
    }

    for i in idx..arr.len() {
        arr.swap(i, idx);
        backtrack(arr, idx + 1, res);
        arr.swap(i, idx);
    }
}

#[cfg(test)]
mod tests {

    use super::permute;

    #[test]
    fn it_permute() {
        let mut arr = vec![1, 2, 3];
        let exp_res = vec![
            vec![1, 2, 3], 
            vec![1, 3, 2], 
            vec![2, 1, 3], 
            vec![2, 3, 1], 
            vec![3, 2, 1], 
            vec![3, 1, 2]
        ];

        assert_eq!(permute(&mut arr), exp_res);
    }
}