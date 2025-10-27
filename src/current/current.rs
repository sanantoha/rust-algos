
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    0
}


pub fn levenshtein_distance1(s1: &str, s2: &str) -> usize {
    0
}


#[cfg(test)]
mod tests {
    use super::{levenshtein_distance, levenshtein_distance1};

    #[test]
    fn it_levenshtein_distance() {
        assert_eq!(levenshtein_distance("abc", "yabd"), 2);
    }

    #[test]
    fn it_levenshtein_distance1() {
        assert_eq!(levenshtein_distance1("abc", "yabd"), 2);
    }
}