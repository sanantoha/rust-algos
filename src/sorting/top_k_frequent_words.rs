use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// O(n * log(k)) time | O(n) space
pub fn top_k_frequent<'a>(words: &Vec<&'a str>, k: i32) -> Vec<&'a str> {

    let mut words_map: HashMap<&str, Word> = HashMap::new();
    for &w in words {
        let word = words_map.entry(w).or_insert(Word {word: w, cnt: 0});
        word.cnt += 1;
    }

    let mut heap: BinaryHeap<&Word> = BinaryHeap::new();

    let mut i = 0;
    for (_, word) in &words_map {
        heap.push(word);
        if i >= k {
            heap.pop();
        }
        i += 1;
    }

    let mut res = vec![];
    while !heap.is_empty() {
        if let Some(word) = heap.pop() {
            res.push(word.word);
        }
    }
    res.reverse();
    return res;
}

#[derive(Debug, Eq, PartialEq)]
struct Word<'a> {
    word: &'a str,
    cnt: i32,
}

impl Ord for Word<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cnt == other.cnt {
            return self.word.to_lowercase().cmp(&other.word.to_lowercase());
        }
        return other.cnt.cmp(&self.cnt);
    }
}

impl PartialOrd for Word<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other))
    }
}


#[test]
pub fn test_top_k_frequent() {

    let words = vec!["i","love","leetcode","i","love","coding"];

    let words1 = vec!["the","day","is","sunny","the","the","the","sunny","is","is"];

    assert_eq!(top_k_frequent(&words, 2), vec!["i", "love"]);
    assert_eq!(top_k_frequent(&words1, 4), vec!["the","is","sunny","day"]);
}