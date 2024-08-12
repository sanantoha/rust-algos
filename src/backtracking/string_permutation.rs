
// O(n ^ 2 * n!) time | O(n * n!) space
pub fn permute(src: &String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    backtrack(src, "", &mut res);
    return res;
}

fn backtrack(src: &str, ans: &str, res: &mut Vec<String>) {
    if src.is_empty() {
        res.push(ans.to_string());
        return;
    }

    for i in 0..src.len() {
        let rem = String::from(&src[0..i]) +  &src[(i + 1)..];

        if let Some(c) = src.chars().nth(i) {
            let mut nans = String::from(ans);
            nans.push(c);
            backtrack(&rem, &nans, res);
        }
    }
}

#[test]
fn test_string_permutation() {
    let src = String::from("abc");

    assert_eq!(permute(&src), vec!["abc", "acb", "bac", "bca", "cab", "cba"])
}