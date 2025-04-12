
// O(s1 * s2 * min(s1 * s2)) time | O(s1 * s2 * min(s1 * s2)) space
pub fn longest_common_subsequence(str1: &str, str2: &str) -> Vec<char> {
    if str1.is_empty() || str2.is_empty() {
        return vec![];
    }

    let mut lcs = vec![vec![vec![]; str2.len() + 1]; str1.len() + 1];

    for i in 1..=str1.len() {
        for j in 1..=str2.len() {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                let mut v = lcs[i - 1][j - 1].clone();
                if let Some(c) = str1.chars().nth(i - 1) {
                    v.push(c);
                }
                lcs[i][j] = v;
            } else {
                let up = lcs[i - 1][j].clone();
                let left = lcs[i][j - 1].clone();
                if up.len() > left.len() {
                    lcs[i][j] = up;
                } else {
                    lcs[i][j] = left;
                }
            }
        }
    }

    lcs[str1.len()].pop().unwrap_or_default()
}

// O(s1 * s2) time | O(s1 * s2) space
pub fn longest_common_subsequence1(str1: &str, str2: &str) -> Vec<char> {
    if str1.is_empty() || str2.is_empty() {
        return vec![];
    }

    let mut lcs = vec![vec![Info::default(); str2.len() + 1]; str1.len() + 1];

    for i in 1..=str1.len() {
        for j in 1..=str2.len() {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                lcs[i][j] = Info {
                    prev_x: Some(i - 1),
                    prev_y: Some(j - 1),
                    l: lcs[i - 1][j - 1].l + 1,
                    c: str1.chars().nth(i - 1).unwrap(),
                }
            } else {
                if lcs[i - 1][j].l > lcs[i][j - 1].l {
                    lcs[i][j] = lcs[i - 1][j].clone();
                } else {
                    lcs[i][j] = lcs[i][j - 1].clone();
                }
            }
        }
    }

    reconstruct(&lcs)
}

fn reconstruct(lcs: &[Vec<Info>]) -> Vec<char> {
    let mut oi = Some(lcs.len() - 1);
    let mut oj = Some(lcs[0].len() - 1);

    let mut res = vec![];

    while let (Some(i), Some(j)) = (oi.take(), oj.take()) {
        let info = &lcs[i][j];
        if info.c != '\0' {
            res.push(info.c);
        }

        oi = info.prev_x;
        oj = info.prev_y;
    }

    res.reverse();
    res
}

#[derive(Debug, Default, Clone)]
struct Info {
    prev_x: Option<usize>,
    prev_y: Option<usize>,
    l: usize,
    c: char,
}

#[cfg(test)]
mod tests {
    use crate::dynamic::longest_common_subsequence::{longest_common_subsequence, longest_common_subsequence1};

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(longest_common_subsequence("ZXVVYZW", "XKYKZPW"), vec!['X', 'Y', 'Z', 'W']);
    }

    #[test]
    fn test_longest_common_subsequence1() {
        assert_eq!(longest_common_subsequence1("ZXVVYZW", "XKYKZPW"), vec!['X', 'Y', 'Z', 'W']);
    }
}