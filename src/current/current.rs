pub fn can_finish(num_courses: usize, prerequisites: &[&[usize]]) -> bool {
    false
}


pub fn can_finish1(num_courses: usize, prerequisites: &[&[usize]]) -> bool {
    false
}


#[cfg(test)]
mod tests {
    use super::{can_finish, can_finish1};

    #[test]
    fn test_can_finish_case1() {
        let prerequisites: &[&[usize]] = &[];
        assert!(can_finish(1, &prerequisites));
    }

    #[test]
    fn test_can_finish_case2() {
        let prerequisites: &[&[usize]] = &[&[1, 0]];
        assert!(can_finish(2, &prerequisites));
    }

    #[test]
    fn test_can_finish_case3() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[0, 1]];
        assert!(!can_finish(2, &prerequisites));
    }

    #[test]
    fn test_can_finish_case4() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2], &[0, 3]];
        assert!(!can_finish(4, &prerequisites));
    }

    #[test]
    fn test_can_finish_case5() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2]];
        assert!(can_finish(4, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case1() {
        let prerequisites: &[&[usize]] = &[];
        assert!(can_finish1(1, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case2() {
        let prerequisites: &[&[usize]] = &[&[1, 0]];
        assert!(can_finish1(2, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case3() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[0, 1]];
        assert!(!can_finish1(2, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case4() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2], &[0, 3]];
        assert!(!can_finish1(4, &prerequisites));
    }

    #[test]
    fn test_can_finish1_case5() {
        let prerequisites: &[&[usize]] = &[&[1, 0], &[2, 1], &[3, 2]];
        assert!(can_finish1(4, &prerequisites));
    }
}