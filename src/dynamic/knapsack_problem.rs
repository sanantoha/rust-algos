
// O(w * h) time | O(w * h) space
pub fn knapsack_problem(items: &[(usize, usize)], capacity: usize) -> Vec<Vec<usize>> {
    if items.is_empty() || capacity <= 0 {
        return vec![];
    }

    let mut kp = vec![vec![0; capacity + 1]; items.len() + 1];

    for i in 1..=items.len() {
        for c in 1..=capacity {
            let (val, weight) = items[i - 1];
            if c < weight {
                kp[i][c] = kp[i - 1][c];
            } else {
                kp[i][c] = kp[i - 1][c].max(kp[i - 1][c - weight] + val);
            }
        }
    }

    vec![vec![kp[items.len()][capacity]], get_final_items(items, kp)]
}

fn get_final_items(items: &[(usize, usize)], kp: Vec<Vec<usize>>) -> Vec<usize> {
    let mut i = kp.len() - 1;
    let mut c = kp[i].len() - 1;

    let mut res = vec![];

    while i > 0 {
        if kp[i][c] == kp[i - 1][c] {
            i -= 1;
        } else {
            res.push(i - 1);
            c -= items[i - 1].1;
        }

        if c == 0 {
            break;
        }
    }

    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use crate::dynamic::knapsack_problem::knapsack_problem;

    #[test]
    fn it_knapsack_problem() {

        let items: &[(usize, usize)] = &[(1, 2), (4, 3), (5, 6), (6, 7), (7, 8)]; // 0 - value, 1 - weight
        let capacity = 10;

        let exp: Vec<Vec<usize>> = vec![vec![10], vec![1, 3]];

        let res = knapsack_problem(items, capacity);
        println!("{:?}", res);
        assert_eq!(res, exp);
    }
}