
pub fn find_ladders<'a>(begin_word: &'a str, end_word: &'a str, word_list: Vec<&'a str>) -> Vec<Vec<&'a str>> {

    vec![]
}


#[cfg(test)]
mod tests {
    use super::find_ladders;

    #[test]
    fn test_find_ladders() {
        let begin_word = "hit";
        let end_word = "cog";
        let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"];

        let res = find_ladders(begin_word, end_word, word_list);
        println!("{:?}", res);

        assert_eq!(res, vec![vec!["hit", "hot", "dot", "dog", "cog"], vec!["hit", "hot", "lot", "log", "cog"]]);
    }
}