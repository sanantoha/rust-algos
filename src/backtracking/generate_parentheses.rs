use std::vec;

// O((4 ^ n) / sqrt(n)) time | O((4 ^ n) / sqrt(n)) space // found in leetcode https://leetcode.com/problems/generate-parentheses/solution/
pub fn generate_parentheses(cnt: i32) -> Vec<String> {
    let mut res = vec![];
    backtrack(cnt, 0, 0, "", &mut res);    
    res
}

fn backtrack(cnt: i32, open: i32, close: i32, ans: &str, res: &mut Vec<String>) {
    if cnt * 2 == open + close {
        res.push(ans.to_owned());
        return;
    }

    if open < cnt {
        backtrack(cnt, open + 1, close, &format!("{}{}", ans, '('), res);
    }
    if close < open {
        backtrack(cnt, open, close + 1, &format!("{}{}", ans, ')'), res);
    }
}

#[cfg(test)]
mod tests {

    use super::generate_parentheses;

    #[test]
    fn it_generate_parentheses() {

        assert_eq!(generate_parentheses(3), vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}