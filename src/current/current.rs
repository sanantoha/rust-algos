
pub fn lnds(arr: &[i32]) -> i32 {
    0
}

pub fn lnds1(arr: &[i32]) -> i32 {
    0
}

pub fn lnds_list(arr: &[i32]) -> Vec<i32> {
    vec![]
}

pub fn lnds_list1(arr: &[i32]) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {

    use super::{lnds, lnds1, lnds_list, lnds_list1};

    const ARR: &[i32] = &[-2, -1, 2, 3, 4, 5, 2, 2, 2, 2, 3];
    #[test]
    fn test_lnds() {
        assert_eq!(lnds(ARR), 8);
    }

    #[test]
    fn test_lnds1() {
        assert_eq!(lnds1(ARR), 8);
    }

    #[test]
    fn test_lnds_list() {
        assert_eq!(lnds_list(ARR), vec![-2, -1, 2, 2, 2, 2, 2, 3]);
    }

    #[test]
    fn test_lnds_list1() {
        assert_eq!(lnds_list1(ARR), vec![-2, -1, 2, 2, 2, 2, 2, 3]);
    }
}