use crate::tree::TreeNode;

pub fn permute(arr: &mut [i32]) -> Vec<Vec<i32>> {
    vec![]
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