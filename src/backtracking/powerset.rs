
// O(n * 2 ^ n) time | O(n * 2 ^ n) space
pub fn powerset(arr: &[i32]) -> Vec<Vec<i32>> {

    let mut res = vec![];
    res.push(vec![]);

    for num in arr {
        let mut sub_res = vec![];
        for lst in &res {
            let mut nlst = lst.clone();
            nlst.push(*num);
            sub_res.push(nlst);
        }
        res.append(&mut sub_res);
    }

    res
}

// O(n * 2 ^ n) time | O(n * 2 ^ n) space
pub fn powerset_rec(arr: &[i32]) -> Vec<Vec<i32>> {
    helper(arr, arr.len() as i64 - 1)
}

fn helper(arr: &[i32], idx: i64) -> Vec<Vec<i32>> {
    if idx < 0 {
        let mut res = vec![];
        res.push(vec![]);
        return res;
    }

    let num = arr[idx as usize];

    let mut sub_res = helper(arr, idx - 1);
    let size = sub_res.len();

    for i in 0..size {
        let mut nlst = sub_res[i].clone();
        nlst.push(num);
        sub_res.push(nlst);
    }

    sub_res
}

#[cfg(test)]
mod tests {
    use crate::backtracking::powerset::powerset;
    use crate::backtracking::powerset::powerset_rec;

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