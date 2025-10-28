
pub fn reverse(i: i32) -> i32 {
    0
}


#[cfg(test)]
mod tests {

    use super::reverse;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_reverse_case1() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn test_reverse_case2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_reverse_case3() {
        assert_eq!(reverse(120), 21);
    }
}