
pub fn string_without_aaa_bbb(a: i32, b: i32) -> String {
    "".to_owned()
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