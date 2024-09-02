
// O(s1 ^ 2 * s1! + s1! * s2) time | O(s1 * s1!) space
pub fn find_permutation(str1: &str, str2: &str) -> bool {
    for s in permute(str1) {
        if str2.contains(&s) {
            return true; 
        }
    }
    false
}

fn permute(src: &str) -> Vec<String> {
    let mut res = vec![];
    backtrack(src, "", &mut res);
    res
}

fn backtrack(src: &str, ans: &str, res: &mut Vec<String>) {
    if src.is_empty() {
        res.push(ans.to_string());
        return;
    }

    for i in 0..src.len() {
        let rem = String::from(&src[0..i]) + &src[i + 1..];

        if let Some(c) = src.chars().nth(i) {
            let mut ans_str = String::from(ans);
            ans_str.push(c);
            backtrack(&rem, &ans_str, res);
        }
    }
}

// O(s1 + s2) time | O(1) space
pub fn find_permutation1(str1: &str, str2: &str) -> bool {

    let mut alphas: [i32; 26]  = [0; 26];

    let min_val = 'a' as usize;

    for i in 0..str1.len() {
        if let Some(c) = str1.chars().nth(i) {
            alphas[c as usize - min_val] += 1;
        }
    }

    for i in 0..str2.len() {
        if let Some(c) = str2.chars().nth(i) {
            alphas[c as usize - min_val] -= 1;

            if i >= str1.len() {
                if let Some(cc) = str2.chars().nth(i - str1.len()) {
                    alphas[cc as usize - min_val] += 1;
                }                
            }
        }

        if all_zeros(alphas) {
            return true;
        }
    }
    false
}

fn all_zeros(alphas: [i32; 26]) -> bool {
    for v in alphas {
        if v != 0 {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {

    use super::find_permutation;
    use super::find_permutation1;

    #[test]
    fn it_find_permutation() {

        assert!(find_permutation("abc", "hdflebacworld"));
    }

    #[test]
    fn it_find_permutation_case2() {

        assert!(find_permutation("abbc", "hbbcadflebdworld"));
    }

    #[test]
    fn it_find_permutation1() {

        assert!(find_permutation1("abc", "hdflebacworld"));
    }

    #[test]
    fn it_find_permutation1_case2() {

        assert!(find_permutation1("abbc", "hbbcadflebdworld"));
    }
}