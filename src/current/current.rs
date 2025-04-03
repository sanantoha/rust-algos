
pub fn ladder_length(begin_word: &str, end_word: &str, word_list: &[&str]) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::ladder_length;

    #[test]
    pub fn test_ladder_length() {

        assert_eq!(ladder_length("hit", "cog", &["hot","dot","dog","lot","log","cog"]), 5);
    }

    #[test]
    pub fn test_ladder_length_without_end_word() {
        assert_eq!(ladder_length("hit", "cog", &["hot","dot","dog","lot","log"]), 0);
    }

    #[test]
    pub fn test_ladder_length_mama() {

        assert_eq!(ladder_length("MAMA", "SIRI", &["SAMA", "SIMA", "SIRA", "SIRI", "MISA", "DISA"]), 5);
    }
}