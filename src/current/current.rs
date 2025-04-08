use std::collections::HashMap;


pub fn optimal_freelancing(jobs: &mut Vec<HashMap<&str, i32>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::optimal_freelancing;
    use std::collections::HashMap;

    #[test]
    fn it_optimal_freelancing() {

        let mut jobs: Vec<HashMap<&str, i32>> = vec![
            HashMap::from([("deadline", 2), ("payment", 2)]),
            HashMap::from([("deadline", 4), ("payment", 3)]),
            HashMap::from([("deadline", 5), ("payment", 1)]),
            HashMap::from([("deadline", 7), ("payment", 2)]),
            HashMap::from([("deadline", 3), ("payment", 1)]),
            HashMap::from([("deadline", 3), ("payment", 2)]),
            HashMap::from([("deadline", 1), ("payment", 3)]),
        ];

        assert_eq!(optimal_freelancing(&mut jobs), 13);
    }
}