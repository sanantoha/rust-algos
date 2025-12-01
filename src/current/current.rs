
pub fn find_permutation(s1: &str, s2: &str) -> bool {
    false
}

pub fn find_permutation1(s1: &str, s2: &str) -> bool {
    false
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