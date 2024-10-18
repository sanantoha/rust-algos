
// O(n) time | O(1) space
pub fn one_edit(s1: &str, s2: &str) -> bool {
    let l1 = s1.len();
    let l2 = s2.len();
    if l1.max(l2) - l1.min(l2) > 1 {
        return false;
    }

    let mut one_edit = false;

    let mut i = 0;
    let mut j = 0;

    while i < l1 && j < l2 {
        if s1.as_bytes()[i] != s2.as_bytes()[j] {
            if one_edit {
                return false;
            }
            one_edit = true;

            if l1 > l2 {
                i += 1;
            } else if l1 < l2 {
                j += 2;
            } else {
                i += 1;
                j += 1;
            }
        } else {
            i += 1;
            j += 1;
        }
    }

    one_edit
}

#[cfg(test)]
mod tests {
    use crate::strings::one_edit::one_edit;

    #[test]
    fn test_one_edit() {
        let s1 = String::from("hello");
        let s2 = String::from("helo");

        assert!(one_edit(&s1, &s2));
    }

    #[test]
    fn test_one_edit_case1() {
        let s1 = String::from("hello");
        let s2 = String::from("hdlo");

        assert!(!one_edit(&s1, &s2));
    }
}