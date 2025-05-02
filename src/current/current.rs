
pub fn lds(arr: &[i32]) -> i32 {
    0
}

pub fn lds1(arr: &[i32]) -> i32 {
    0
}

pub fn lds_list(arr: &[i32]) -> Vec<i32> {
    vec![]
}

pub fn lds_list1(arr: &[i32]) -> Vec<i32> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::{lds, lds1, lds_list, lds_list1};

    const ARR0: &[i32] = &[5,6,7,6,5,4,3,10,14,12];

    const ARR1: &[i32] = &[100, 10, 9, 8, 7, 6, 5, 90, 80, 70, 60, 50, 40, 30, 20];

    #[test]
    fn test_lds_case0() {
        assert_eq!(lds(ARR0), 5);
    }

    #[test]
    fn test_lds_case1() {
        assert_eq!(lds(ARR1), 9);
    }

    #[test]
    fn test_lds1_case0() {
        assert_eq!(lds1(ARR0), 5);
    }

    #[test]
    fn test_lds1_case1() {
        assert_eq!(lds1(ARR1), 9);
    }

    #[test]
    fn test_lds_list_case0() {
        assert_eq!(lds_list(ARR0), vec![7, 6, 5, 4, 3]);
    }

    #[test]
    fn test_lds_list_case1() {
        assert_eq!(lds_list(ARR1), vec![100, 90, 80, 70, 60, 50, 40, 30, 20]);
    }

    #[test]
    fn test_lds_list1_case0() {
        assert_eq!(lds_list1(ARR0), vec![7, 6, 5, 4, 3]);
    }

    #[test]
    fn test_lds_list1_case1() {
        assert_eq!(lds_list1(ARR1), vec![100, 90, 80, 70, 60, 50, 40, 30, 20]);
    }
}