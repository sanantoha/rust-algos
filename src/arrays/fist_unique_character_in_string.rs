use std::collections::HashMap;

// O(n) time | O(n) space
pub fn first_unique_character_in_string(src: &str) -> Option<usize> {

    let mut map = HashMap::new();

    for c in src.chars() {
        let cnt = map.entry(c).or_insert(0);
        *cnt += 1;        
    }

    for (i, c) in src.chars().enumerate() {
        if map.get(&c).copied().unwrap_or_default() == 1 {
            return Some(i);
        }
    }
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