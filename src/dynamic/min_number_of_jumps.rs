
// O(n ^ 2) time | O(n) space
pub fn min_number_of_jumps(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut mnj = vec![i32::MAX; arr.len()];
    mnj[0] = 0;

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] + j as i32 >= i  as i32 && mnj[i] > mnj[j] + 1 {
                mnj[i] = mnj[j] + 1;
            }
        }
    }

    mnj[mnj.len()-1]
}

// O(n) time | O(1) space
pub fn min_number_of_jumps1(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut jumps = 0;
    let mut max_reach = arr[0];
    let mut steps = arr[0];

    for i in 0..arr.len() {
        max_reach = max_reach.max(arr[i] + i as i32);
        steps -= 1;
        if steps == 0 {
            jumps += 1;
            steps = max_reach - i as i32;
        }
    }

    jumps + 1
}

#[cfg(test)]
mod tests {
    use crate::dynamic::min_number_of_jumps::{min_number_of_jumps, min_number_of_jumps1};

    const ARR: &[i32] = &[3, 4, 2, 1, 2, 3, 7, 1, 1, 1, 3];

    #[test]
    fn test_min_number_of_jumps() {

        let res = min_number_of_jumps(ARR);
        println!("{}", res);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_min_number_of_jumps1() {
        let res = min_number_of_jumps1(ARR);
        println!("{}", res);
        assert_eq!(res, 4);
    }
}