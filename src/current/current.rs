


pub fn first_unique_character_in_string(src: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::first_unique_character_in_string;

    #[test]
    pub fn test_first_unique_character_in_string() {

        assert_eq!(first_unique_character_in_string("leetcode"), Some(0));
        assert_eq!(first_unique_character_in_string("loveleetcode"), Some(2));
        assert_eq!(first_unique_character_in_string("aabb"), None);
    }
}