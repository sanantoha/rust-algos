
pub fn one_edit(s1: &str, s2: &str) -> bool {
    false
}


#[cfg(test)]
mod tests {
    use super::one_edit;

    #[test]
    fn test_one_edit() {
        let s1 = String::from("hello");
        let s2 = String::from("helo");

        assert!(one_edit(&s1, &s2));
    }

    #[test]
    fn test_one_edit_case1() {
        let s1 = String::from("hello");
        let s2 = String::from("hdlo");

        assert!(!one_edit(&s1, &s2));
    }
}