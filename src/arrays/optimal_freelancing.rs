use std::collections::HashMap;

// O(n * log(n)) time | O(1) space
pub fn optimal_freelancing(jobs: &mut Vec<HashMap<&str, i32>>) -> i32 {
    if jobs.is_empty() {
        return 0;
    }
    
    jobs.sort_by(|x, y| {
        let payment_x = x.get("payment").unwrap_or(&0);
        let payment_y = y.get("payment").unwrap_or(&0);
        payment_y.cmp(payment_x)
    });

    let mut max_profit = 0;

    const LENGTH_OF_WEEK: usize = 7;
    let mut timeline = [false; LENGTH_OF_WEEK];

    for job in jobs {
        if let Some(t) = job.get("deadline") {
            let max_time = LENGTH_OF_WEEK.min(*t as usize);

            for time in (0..max_time).rev() {
                if !timeline[time] {
                    timeline[time] = true;
                    if let Some(&payment) = job.get("payment") {
                        max_profit += payment;
                    }
                    break;
                }
            }
        }
    }
    
    max_profit
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