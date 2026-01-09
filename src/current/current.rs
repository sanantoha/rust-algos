pub fn next_greater_element(arr: &[i32]) -> Vec<i32> {
    vec![]
}

pub fn next_greater_element1(arr: &[i32]) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR: &[i32] = &[2, 5, -3, -4, 6, 7, 2];

    #[test]
    fn test_next_greater_element() {
        assert_eq!(next_greater_element(ARR), vec![5, 6, 6, 6, 7, -1, 5])
    }

    #[test]
    fn test_next_greater_element1() {
        assert_eq!(next_greater_element1(ARR), vec![5, 6, 6, 6, 7, -1, 5])
    }
}