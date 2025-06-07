

pub fn generate_parentheses(cnt: i32) -> Vec<String> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::generate_parentheses;

    #[test]
    fn it_generate_parentheses() {

        assert_eq!(generate_parentheses(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}