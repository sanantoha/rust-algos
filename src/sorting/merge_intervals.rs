use std::collections::VecDeque;

// O(n * log(n)) time | O(n) space
pub fn merge(intervals: &mut [(i32, i32)]) -> Vec<(i32, i32)> {
    if intervals.len() <= 1 {
        return intervals.to_vec();
    }

    intervals.sort_by_key(|k| k.0);

    let mut merged = VecDeque::new();

    for interval in intervals {
        if let Some((_, x)) = merged.back_mut() {
            if *x < interval.0 {
                merged.push_back(*interval);
            } else {
                *x = interval.1.max(*x);
            }
        } else {
            merged.push_back(*interval);
        }
    }

    merged.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn test_merge() {
        let mut intervals: Vec<(i32, i32)> = vec![
            (1, 5),
            (3, 7),
            (4, 6),
            (6, 8),
        ];


        let res = merge(&mut intervals);
        println!("{:?}", res);
        assert_eq!(res, vec![(1, 8)]);
    }

    #[test]
    fn test_merge_case1() {
        let mut intervals: Vec<(i32, i32)> = vec![
            (1, 3),
            (2, 6),
            (8, 10),
            (15, 18),
        ];

        let res = merge(&mut intervals);
        println!("{:?}", res);
        assert_eq!(res, vec![(1, 6), (8, 10), (15, 18)]);
    }

    #[test]
    fn test_merge_case2() {
        let mut intervals: Vec<(i32, i32)> = vec![
            (1, 4),
            (4, 5),
        ];

        let res = merge(&mut intervals);
        println!("{:?}", res);
        assert_eq!(res, vec![(1, 5)]);
    }
}