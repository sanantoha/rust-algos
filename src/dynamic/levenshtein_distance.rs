
// O(a * b) time | O(a * b) space
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut ld = vec![vec![0usize; b.len() + 1]; a.len() + 1];

    for i in 1..=a.len() {
        ld[i][0] = i;
    }

    for i in 1..=b.len() {
        ld[0][i] = i;
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                ld[i][j] = ld[i - 1][j - 1];
            } else {
                ld[i][j] = 1 + ld[i - 1][j].min(ld[i][j - 1]).min(ld[i - 1][j - 1]);
            }
        }
    }

    ld[a.len()][b.len()]
}

// O(a * b) time | O(min(a, b)) space
pub fn levenshtein_distance1(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let small = if a.len() < b.len() {
        a
    } else {
        b
    };
    let big = if a.len() < b.len() {
        b
    } else {
        a
    };

    let mut prev = vec![0; small.len() + 1]; // even
    for i in 1..=small.len() {
        prev[i] = i;
    }
    let mut curr = vec![0; small.len() + 1]; // odd

    for i in 1..=big.len() {
        curr[0] = i;
        for j in 1..=small.len() {
            if big.as_bytes()[i - 1] == small.as_bytes()[j - 1] {
                curr[j] = prev[j - 1];
            } else {
                curr[j] = 1 + curr[j - 1].min(prev[j - 1]).min(prev[j]);
            }
        }
        std::mem::swap(&mut curr, &mut prev);
    }

    if big.len() % 2 == 0 {
        prev[small.len()]
    } else {
        curr[small.len()]
    }
    //    a b c
    //  0 1 2 3
    //y 1 1 2 3
    //a 2 1 2 3
    //b 3 2 1 2
    //d 4 3 2 2
}

#[cfg(test)]
mod tests {
    use crate::dynamic::levenshtein_distance::{levenshtein_distance, levenshtein_distance1};

    #[test]
    fn it_levenshtein_distance() {
        assert_eq!(levenshtein_distance("abc", "yabd"), 2);
    }

    #[test]
    fn it_levenshtein_distance1() {
        assert_eq!(levenshtein_distance1("abc", "yabd"), 2);
    }
}

