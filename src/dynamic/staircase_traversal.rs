
// O(max_steps ^ height) time | O(height) space
pub fn staircase_traversal_rec(height: usize, max_steps: usize) -> i32 {
    if height <= 0 {
        return 1;
    }

    let mut cnt = 0;

    for i in 1..=height.min(max_steps) {
        cnt += staircase_traversal_rec(height - i, max_steps);
    }
    cnt
}

// O(max_steps * height) time | O(height) space
pub fn staircase_traversal_rec_memoization(height: usize, max_steps: usize) -> i32 {
    let mut memo = vec![0; height + 1];
    memo[0] = 1;
    helper(height, max_steps, &mut memo)
}

fn helper(height: usize, max_steps: usize, memo: &mut Vec<i32>) -> i32 {
    if memo[height] != 0 {
        return memo[height];
    }

    let mut cnt = 0;

    for i in 1..=max_steps.min(height) {
        cnt += helper(height - i, max_steps, memo);
    }

    memo[height] = cnt;
    memo[height]
}

// O(max_steps * height) time | O(height) space
pub fn staircase_traversal_iter(height: usize, max_steps: usize) -> i32 {
    if height <= 1 {
        return 1;
    }

    let mut ways_to_tap = vec![0; height + 1];
    ways_to_tap[0] = 1;

    for i in 1..=height {
        let mut step = 1;
        while step <= max_steps && i >= step {
            ways_to_tap[i] += ways_to_tap[i - step];
            step += 1;
        }
    }

    ways_to_tap[height]
}

// O(height) time | O(height) space
pub fn staircase_traversal_sliding_window(height: usize, max_steps: usize) -> i32 {
    if height <= 1 {
        return 1;
    }

    let mut ways_to_tap = vec![0; height + 1];
    ways_to_tap[0] = 1;

    let mut sum = 0;

    for i in 1..=height {
        if i > max_steps {
            let start_idx = i - max_steps - 1; // always greater or equal to 0, because i > max_steps
            sum -= ways_to_tap[start_idx];
        }
        let end_idx = i - 1;
        sum += ways_to_tap[end_idx];

        ways_to_tap[i] = sum;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::dynamic::staircase_traversal::{staircase_traversal_iter, staircase_traversal_rec, staircase_traversal_rec_memoization, staircase_traversal_sliding_window};

    #[test]
    fn test_staircase_traversal_rec() {
        assert_eq!(staircase_traversal_rec(4, 2), 5);
    }

    #[test]
    fn test_staircase_traversal_rec_case1() {
        assert_eq!(staircase_traversal_rec(4, 3), 7);
    }

    #[test]
    fn test_staircase_traversal_rec_memoization() {
        assert_eq!(staircase_traversal_rec_memoization(4, 2), 5);
    }

    #[test]
    fn test_staircase_traversal_rec_memoization_case1() {
        assert_eq!(staircase_traversal_rec_memoization(4, 3), 7);
    }

    #[test]
    fn test_staircase_traversal_iter() {
        assert_eq!(staircase_traversal_iter(4, 2), 5);
    }

    #[test]
    fn test_staircase_traversal_iter_case1() {
        assert_eq!(staircase_traversal_iter(4, 3), 7);
    }

    #[test]
    fn test_staircase_traversal_sliding_window() {
        assert_eq!(staircase_traversal_sliding_window(4, 2), 5);
    }

    #[test]
    fn test_staircase_traversal_sliding_window_case1() {
        assert_eq!(staircase_traversal_sliding_window(4, 3), 7);
    }
}