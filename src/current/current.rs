
pub fn top_k_frequent<'a>(_words: &[&'a str], _k: i32) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {

    use super::top_k_frequent;

    #[test]
    pub fn test_top_k_frequent() {
        let words = vec!["i","love","leetcode","i","love","coding"];

        assert_eq!(top_k_frequent(&words, 2), vec!["i", "love"]);
    }

    #[test]
    pub fn test_top_k_frequent_for_4() {
        let words = vec!["the","day","is","sunny","the","the","the","sunny","is","is"];

        assert_eq!(top_k_frequent(&words, 4), vec!["the","is","sunny","day"]);
    }
}