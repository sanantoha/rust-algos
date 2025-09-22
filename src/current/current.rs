
pub fn lis0(arr: &[i32]) -> i32 {
    0
}

pub fn lis(arr: &[i32]) -> i32 {
    0
}

pub fn lis_list0(arr: &[i32]) -> Vec<i32> {
    vec![]
}

pub fn lis_list(arr: &[i32]) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::{lis0, lis, lis_list0, lis_list};

    const ARR0: &[i32] = &[1, 2, 3, 6, -100, -90, -80, -70, -60, 7, 8, 9, 10, -50, -40];

    const ARR1: &[i32] = &[10, 22, 9, 33, 21, 50, 41, 60, 80];

    const ARR2: &[i32] = &[4,10,4,3,8,9];

    const ARR3: &[i32] = &[10,9,2,5,3,7,101,18];

    const ARR4: &[i32] = &[1, -10, 20, 30, 2, 3, 4, 5];

    #[test]
    fn test_lis0_case0() {
        assert_eq!(lis0(ARR0), 9);
    }

    #[test]
    fn test_lis0_case1() {
        assert_eq!(lis0(ARR1), 6);
    }

    #[test]
    fn test_lis0_case2() {
        assert_eq!(lis0(ARR2), 3);
    }

    #[test]
    fn test_lis0_case3() {
        assert_eq!(lis0(ARR3), 4);
    }

    #[test]
    fn test_lis0_case4() {
        assert_eq!(lis0(ARR4), 5);
    }

    #[test]
    fn test_lis_case0() {
        assert_eq!(lis(ARR0), 9);
    }

    #[test]
    fn test_lis_case1() {
        assert_eq!(lis(ARR1), 6);
    }

    #[test]
    fn test_lis_case2() {
        assert_eq!(lis(ARR2), 3);
    }

    #[test]
    fn test_lis_case3() {
        assert_eq!(lis(ARR3), 4);
    }

    #[test]
    fn test_lis_case4() {
        assert_eq!(lis(ARR4), 5);
    }

    #[test]
    fn test_lis_list0_case0() {
        assert_eq!(lis_list0(ARR0), vec![-100, -90, -80, -70, -60, 7, 8, 9, 10]);
    }

    #[test]
    fn test_lis_list0_case1() {
        assert_eq!(lis_list0(ARR1), vec![10, 22, 33, 50, 60, 80]);
    }

    #[test]
    fn test_lis_list0_case2() {
        assert_eq!(lis_list0(ARR2), vec![4, 8, 9]);
    }

    #[test]
    fn test_lis_list0_case3() {
        assert_eq!(lis_list0(ARR3), vec![2, 5, 7, 101]);
    }

    #[test]
    fn test_lis_list0_case4() {
        assert_eq!(lis_list0(ARR4), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_lis_list_case0() {
        assert_eq!(lis_list(ARR0), vec![-100, -90, -80, -70, -60, 7, 8, 9, 10]);
    }

    #[test]
    fn test_lis_list_case1() {
        assert_eq!(lis_list(ARR1), vec![10, 22, 33, 41, 60, 80]);
    }

    #[test]
    fn test_lis_list_case2() {
        assert_eq!(lis_list(ARR2), vec![3, 8, 9]);
    }

    #[test]
    fn test_lis_list_case3() {
        assert_eq!(lis_list(ARR3), vec![2, 3, 7, 18]);
    }

    #[test]
    fn test_lis_list_case4() {
        assert_eq!(lis_list(ARR4), vec![-10, 2, 3, 4, 5]);
    }
}