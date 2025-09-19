
pub fn min_number_of_jumps(arr: &[i32]) -> i32 {
    0
}

pub fn min_number_of_jumps1(arr: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{min_number_of_jumps, min_number_of_jumps1};

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