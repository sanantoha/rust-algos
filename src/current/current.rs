
pub fn merge(intervals: &mut [(i32, i32)]) -> Vec<(i32, i32)> {
    vec![]
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