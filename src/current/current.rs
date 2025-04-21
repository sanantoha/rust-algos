
pub fn powerset(arr: &[i32]) -> Vec<Vec<i32>> {
    return vec![]
}

pub fn powerset_rec(arr: &[i32]) -> Vec<Vec<i32>> {
    return vec![]
}


#[cfg(test)]
mod tests {
    use super::powerset;
    use super::powerset_rec;

    #[test]
    fn it_powerset() {
        let res = powerset(&[1, 2, 3]);
        println!("{:?}", res);

        assert_eq!(res, vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]]);
    }

    #[test]
    fn it_powerset_rec() {
        let res = powerset_rec(&[1, 2, 3]);
        println!("{:?}", res);

        assert_eq!(res, vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]]);
    }
}