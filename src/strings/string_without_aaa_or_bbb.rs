
// O(a + b) time | O(a + b) space
pub fn string_without_aaa_bbb(mut a: i32, mut b: i32) -> String {

    let mut res = String::new();

    while a > 0 && b > 0 {
        if a > b {
            res.push_str("aab");
            a -= 2;
            b -= 1;
        } else {
            if a == b && a == 1 {
                res.push_str("ab");
                a -= 1;
                b -= 1;
                break;
            }

            res.push_str("abb");
            a -= 1;
            b -= 2;
        }
    }

    while a > 0 {
        res.push_str("a");
        a -= 1;
    }

    while b > 0 {
        res.insert(0, 'b');
        b -= 1;
    }

    res
}

#[cfg(test)]
mod tests {

    use super::string_without_aaa_bbb;

    #[test]
    fn test_string_without_aaa_bbb() {
        assert_eq!(string_without_aaa_bbb(1, 1), "ab");
    }

    #[test]
    fn test_string_without_aaa_bbb_case1() {
        assert_eq!(string_without_aaa_bbb(3, 3), "abbaab");
    }

    #[test]
    fn test_string_without_aaa_bbb_case2() {
        assert_eq!(string_without_aaa_bbb(2, 5), "babbabb");
    }

    #[test]
    fn test_string_without_aaa_bbb_case3() {
        assert_eq!(string_without_aaa_bbb(5, 3), "aabaabab");
    }

    #[test]
    fn test_string_without_aaa_bbb_case4() {
        assert_eq!(string_without_aaa_bbb(3, 3), "abbaab");
    }

    #[test]
    fn test_string_without_aaa_bbb_case5() {
        assert_eq!(string_without_aaa_bbb(1, 4), "bbabb");
    }
}