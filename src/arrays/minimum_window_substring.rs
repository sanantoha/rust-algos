use std::collections::HashMap;


// O(s + t) time | O(s + t) space
pub fn min_window(s: &str, t: &str) -> String {
    if s.is_empty() || t.is_empty() {
        return "".to_owned();
    }

    let mut dict_t = HashMap::new();
    for c in t.chars() {
        let cnt = dict_t.entry(c).or_insert(0);
        *cnt += 1;
    }

    let mut formed = 0;
    let required = dict_t.len();

    let mut l = 0;
    let mut r = 0;

    let mut wc = HashMap::new();

    let mut ans: usize = usize::MAX;
    let mut ans_l = 0;
    let mut ans_r = 0;

    while r < s.len() {
        if let Some(c) = s.chars().nth(r) {
            let cnt = wc.entry(c).or_insert(0);
            *cnt += 1;

            if dict_t.contains_key(&c) && cnt == dict_t.get(&c).unwrap_or(&0) {
                formed += 1;
            }

            while l <= r && formed == required {
                if let Some(c) = s.chars().nth(l) {
                    if ans > r - l + 1 {
                        ans = r - l + 1;
                        ans_l = l;
                        ans_r = r;
                    }
    
                    let cnt = wc.entry(c).or_insert(0);
                    *cnt -= 1;
    
                    if dict_t.contains_key(&c) && wc.get(&c).unwrap_or(&0) < dict_t.get(&c).unwrap_or(&0) {
                        formed -= 1;
                    }
                }                
                l += 1;
            }            
        }    
        r += 1;    
    }

    if ans == usize::MAX {
        "".to_owned()
    } else {
        String::from(&s[ans_l..(ans_r + 1)])
    }    
}


#[cfg(test)]
mod tests {

    use super::min_window;

    #[test]
    fn it_min_window() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC".to_owned());
    }

    #[test]
    fn it_min_window_one_letter() {
        assert_eq!(min_window("a", "a"), "a".to_owned());
    }

    #[test]
    fn it_min_window_imp_result() {
        assert_eq!(min_window("a", "aa"), "".to_owned());
    }
}