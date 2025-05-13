
pub fn search(arr: &[i32], target: i32) -> Result<usize, usize> {
    Err(0)
}

pub fn search1(arr: &[i32], target: i32) -> Result<usize, usize> {
    Err(0)
}

#[cfg(test)]
mod tests {
    use super::{search, search1};

    const ARR: &[i32] = &[40, 50, 60, 70, 80, 90, 0, 10, 20, 30, 31, 32, 33, 34, 35];
    const TARGET: i32 = 90;

    #[test]
    fn test_search() {
        assert_eq!(search(ARR, TARGET), Ok(5));
    }

    #[test]
    fn test_search_case1() {
        assert_eq!(search(ARR, 11), Err(8));
    }

    #[test]
    fn test_search1() {
        assert_eq!(search1(ARR, TARGET), Ok(5));
    }

    #[test]
    fn test_search1_case1() {
        assert_eq!(search1(ARR, 11), Err(8));
    }
}