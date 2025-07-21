
pub fn same_bsts(arr1: &[i32], arr2: &[i32]) -> bool {
    false
}

pub fn same_bsts1(arr1: &[i32], arr2: &[i32]) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{same_bsts, same_bsts1};

    const ARR_ONE: &[i32] = &[10, 15, 8, 12, 94, 81, 5, 2, 11];
    const ARR_TWO: &[i32] = &[10, 8, 5, 15, 2, 12, 11, 94, 81];

    #[test]
    fn test_same_bsts() {
        assert!(same_bsts(ARR_ONE, ARR_TWO));
    }

    #[test]
    fn test_same_bsts1() {
        assert!(same_bsts1(ARR_ONE, ARR_TWO));
    }
}