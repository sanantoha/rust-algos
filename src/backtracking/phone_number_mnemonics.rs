
// O(n * 4 ^ n) time | O(n * 4 ^ n) space
pub fn phone_number_mnemonics(phone_number: &str) -> Vec<String> {
    let mut res = vec![];
    let letters = ["0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    backtrack(phone_number, &letters, 0, "", &mut res);
    res
}

fn backtrack(phone_number: &str, letters: &[&str; 10], idx: usize, ans: &str, res: &mut Vec<String>) {
    if phone_number.len() == ans.len() {
        res.push(ans.to_string());
        return;
    }

    if let Some(c) = phone_number.chars().nth(idx) {
        if let Some(d) = c.to_digit(10) {
            let l = letters[d as usize];

            for i in 0..l.len() {
                if let Some(c) = l.chars().nth(i) {
                    backtrack(phone_number, letters, idx + 1, &format!("{}{}", ans, c), res);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backtracking::phone_number_mnemonics::phone_number_mnemonics;

    #[test]
    fn it_phone_number_mnemonics() {
        let res = phone_number_mnemonics("1905");
        println!("{:?}", res);

        assert_eq!(res, vec!["1w0j", "1w0k", "1w0l", "1x0j", "1x0k", "1x0l",
                                                  "1y0j", "1y0k", "1y0l", "1z0j", "1z0k", "1z0l"]);
    }
}