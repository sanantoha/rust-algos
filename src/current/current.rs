
pub fn balanced_brackets(_str: &str) -> bool {

    false
}

#[cfg(test)]
mod tests {

    use super::balanced_brackets;

    #[test]
    fn it_balanced_brackets() {
        assert!(balanced_brackets("([])(){}(())()()"));
    }

    #[test]
    fn it_im_balanced_brackets() {
        assert!(!balanced_brackets("}{"));
    }
}