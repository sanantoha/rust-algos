

pub fn permute(src: &str) -> Vec<String> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::permute;

    #[test]
    fn test_string_permutation() {
        assert_eq!(permute("abc"), vec!["abc", "acb", "bac", "bca", "cab", "cba"]);
    }
}