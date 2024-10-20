
// O(1) time | O(1) space
pub fn reverse(i: i32) -> i32 {
    let mut res = 0i64;

    let mut j = i;
    while j != 0 {
        res = res * 10 + j as i64 % 10;
        if res < i32::MIN as i64 || res > i32::MAX as i64 {
            return 0;
        }
        j /= 10;
    }

    res as i32
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