
// O(n) time | O(n) space
pub fn rob(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    if arr.len() == 1 {
        return arr[0];
    }

    let mut dp = vec![0; arr.len()];
    dp[0] = arr[0];
    dp[1] = arr[0].max(dp[1]);

    for i in 2..arr.len() {
        dp[i] = dp[i - 1].max(dp[i - 2] + arr[i]);
    }

    dp[arr.len() - 1]
}

// O(n) time | O(1) space
pub fn rob1(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    if arr.len() == 1 {
        return arr[0];
    }

    let mut fst = arr[0];
    let mut snd = arr[0].max(arr[1]);

    for i in 2..arr.len() {
        let tmp = snd.max(arr[i] + fst);
        fst = snd;
        snd = tmp;
    }

    snd
}

#[cfg(test)]
mod tests {
    use crate::dynamic::house_robber::{rob, rob1};

    #[test]
    fn it_rob() {

        assert_eq!(rob(&[4,1,2,7,5,3,1]), 14);
    }

    #[test]
    fn it_rob1() {

        assert_eq!(rob1(&[4,1,2,7,5,3,1]), 14);
    }
}