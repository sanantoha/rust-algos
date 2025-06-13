

pub fn reverse(source: &str) -> String {
    "".to_owned()
}


#[cfg(test)]
mod tests {
    use super::reverse;

    #[test]
    fn test_reverse() {
        let exp = "     example   good a ".to_string();
        assert_eq!(reverse(" a good   example     "), exp);
    }

    #[test]
    fn test_reverse_case1() {
        let exp = "whitespace   of lot     a has     string      this".to_string();
        assert_eq!(reverse("this      string     has a     lot of   whitespace"), exp);
    }
}