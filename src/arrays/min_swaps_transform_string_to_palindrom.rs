
// O(n) time | O(1) space
pub fn min_swaps_required(str: &str) -> Option<i32> {
    if str.is_empty() {
        return None;
    }
    
    let mut l = 0;
    let mut r = str.len() - 1;

    let mut n = 0;

    while l < r {

        if let (Some(lc), Some(rc)) = (str.chars().nth(l), str.chars().nth(r)) {
            if lc != rc {
                n += 1;
            }            
        }

        l += 1;
        r -= 1;
    }
    
    if str.len() % 2 == 0 && n % 2 == 1 {
        None
    } else {
        Some((n + 1) / 2)
    }    
}

#[cfg(test)]
mod tests {

    use super::min_swaps_required;

    #[test]
    fn it_min_swaps_required() {

        assert_eq!(min_swaps_required("0100101"), Some(2));
    }

    #[test]
    fn it_no_way_to_make_palindrom() {

        assert_eq!(min_swaps_required("1110"), None);
    }

    #[test]
    fn it_one_swap() {
        assert_eq!(min_swaps_required("11101"), Some(1));
    }
}