
// O(n) time | O(1) space
pub fn reverse(str: &str) -> String {
    let mut arr: Vec<char> = str.chars().collect();
    let l = arr.len();
    rev(&mut arr, 0, l - 1);

    let mut start_idx = 0;
    while start_idx < arr.len() {
        let mut end_idx = start_idx;
        while end_idx < arr.len() && arr[end_idx] != ' ' {
            end_idx += 1;
        }
        if end_idx > 0 {
            rev(&mut arr, start_idx, end_idx - 1);
        }
        start_idx = end_idx + 1;
    }

    String::from_iter(arr)
}

fn rev(arr: &mut [char], mut l: usize, mut r: usize) {
    while l < r {
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::reverse;

    #[test]
    fn test_reverse() {
        let exp = "     example   good a ".to_string();
        assert_eq!(reverse(" a good   example     "), exp);
    }

    #[test]
    fn test_reverse_case1() {
        let exp = "whitespace   of lot     a has     string      this".to_string();
        assert_eq!(reverse("this      string     has a     lot of   whitespace"), exp);
    }
}