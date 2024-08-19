use std::collections::HashMap;

// O(n) time | O(n) space
pub fn first_unique_character_in_string(src: &str) -> i32 {

    let mut map = HashMap::new();

    for c in src.chars() {
        let cnt = map.entry(c).or_insert(0);
        *cnt += 1;        
    }

    for (i, c) in src.chars().enumerate() {
        if map.get(&c).unwrap_or(&0) == &1 {
            return i as i32;
        }
    }
    -1
}

#[test]
pub fn test_first_unique_character_in_string() {

    assert_eq!(first_unique_character_in_string("leetcode"), 0);
    assert_eq!(first_unique_character_in_string("loveleetcode"), 2);
    assert_eq!(first_unique_character_in_string("aabb"), -1);
}