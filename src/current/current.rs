
pub fn min_window(s: &str, t: &str) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use crate::arrays::minimum_window_substring::min_window;
    use super::min_window;

    #[test]
    fn it_min_window() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC".to_owned());
    }

    #[test]
    fn it_min_window_one_letter() {
        assert_eq!(min_window("a", "a"), "a".to_owned());
    }

    #[test]
    fn it_min_window_imp_result() {
        assert_eq!(min_window("a", "aa"), "".to_owned());
    }
}