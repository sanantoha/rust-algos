
// O(n ^ 2) time | O(n) space
pub fn disk_stacking(disks: &[(i32, i32, i32)]) -> Vec<(i32, i32, i32)> {
    if disks.is_empty() {
        return vec![];
    }

    let mut sorted = disks.to_vec();
    sorted.sort_by_key(|(_, _, h)| *h);

    let mut heights: Vec<i32> = sorted.iter().map(|(_, _, h)| h).copied().collect();
    let mut prev = vec![None; sorted.len()];

    let mut max_idx = 0;

    for i in 1..sorted.len() {
        for j in 0..i {
            if less(&sorted[j], &sorted[i]) && heights[i] < heights[j] + sorted[i].2 {
                heights[i] = heights[j] + sorted[i].2;
                prev[i] = Some(j);
            }
        }

        if heights[max_idx] < heights[i] {
            max_idx = i;
        }
    }


    build_seq(&sorted, &prev, max_idx)
}

fn build_seq(sorted: &[(i32, i32, i32)], prev: &[Option<usize>], max_id: usize) -> Vec<(i32, i32, i32)> {
    let mut res = vec![];
    let mut idx = Some(max_id);

    while let Some(i) = idx.take() {
        res.push(sorted[i]);
        idx = prev[i];
    }

    res.reverse();
    res
}

fn less(d1: &(i32, i32, i32), d2: &(i32, i32, i32)) -> bool {
    d1.0 < d2.0 && d1.1 < d2.1 && d1.2 < d2.2
}

#[cfg(test)]
mod tests {
    use crate::dynamic::disk_stacking::disk_stacking;

    #[test]
    fn test_disk_stacking() {
        let disks: &[(i32, i32, i32)] = &[
            (2, 1, 2),
            (3, 2, 3),
            (2, 2, 8),
            (2, 3, 4),
            (2, 2, 1),
            (4, 4, 5),
        ];

        let exp: Vec<(i32, i32, i32)> = vec![
            (2, 1, 2),
            (3, 2, 3),
            (4, 4, 5)
        ];

        let res = disk_stacking(disks);
        println!("{:?}", res);
        assert_eq!(res, exp);
    }
}