
// O(n ^ 2) time | O(n) space
pub fn min_rewards(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut rewards = vec![1; arr.len()];

    for i in 1..arr.len() {
        if arr[i - 1] < arr[i] {
            rewards[i] = rewards[i - 1] + 1;
        } else if let Some(mut j) = i.checked_sub(1) {
            while arr[j] > arr[j + 1] {
                rewards[j] = rewards[j].max(rewards[j + 1] + 1);
                if let Some(k) = j.checked_sub(1) {
                    j = k;
                } else {
                    break;
                }
            }
        }                    
    }
    rewards.iter().sum()
}

// O(n) time | O(n) space
pub fn min_rewards1(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut rewards = vec![1; arr.len()];

    for idx in get_min_idxs(arr) {
        extend_idx(arr, idx, &mut rewards);
    }

    rewards.iter().sum()
}

fn extend_idx(arr: &[i32], idx: usize, rewards: &mut [i32]) {

    if let Some(mut l) = idx.checked_sub(1) {
        while arr[l] > arr[l + 1] {
            rewards[l] = rewards[l].max(rewards[l + 1] + 1);
            if let Some(k) = l.checked_sub(1) {
                l = k;
            } else {
                break;
            }
        }
    }

    let mut r = idx + 1;
    while r < arr.len() && arr[r - 1] < arr[r] {
        rewards[r] = rewards[r - 1] + 1;
        r += 1;
    }
}

fn get_min_idxs(arr: &[i32]) -> Vec<usize> {
    let mut res = vec![];

    if arr.len() == 1 {
        res.push(0);
        return res;
    }

    for i in 0..arr.len() {
        if i == 0 && arr[i] < arr[i + 1] || arr.len() - 1 == i && arr[i - 1] > arr[i] {
            res.push(i);
        }
        if i == 0 || arr.len() - 1 == i {
            continue;
        }

        if arr[i - 1] > arr[i] && arr[i] < arr[i + 1] {
            res.push(i);
        }

    }

    res
}

// O(n) time | O(n) space
pub fn min_rewards2(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut rewards = vec![1; arr.len()];

    for i in 1..arr.len() {
        if arr[i - 1] < arr[i] {
            rewards[i] = rewards[i - 1] + 1;
        }
    }

    for i in (0..(arr.len() - 1)).rev() {
        if arr[i] > arr[i + 1] {
            rewards[i] = rewards[i].max(rewards[i + 1] + 1);
        }
    }
    
    rewards.iter().sum()
}


#[cfg(test)]
mod tests {

    use super::{min_rewards, min_rewards1, min_rewards2};

    const ARR: &[i32] = &[8, 4, 2, 1, 3, 6, 7, 9, 5];

    #[test]
    fn it_min_rewards() {
        assert_eq!(min_rewards(ARR), 25);
    }

    #[test]
    fn it_min_rewards1() {
        assert_eq!(min_rewards1(ARR), 25);
    }

    #[test]
    fn it_min_rewards2() {
        assert_eq!(min_rewards2(ARR), 25);
    }
}