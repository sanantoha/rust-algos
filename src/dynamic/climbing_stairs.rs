
// O(n) time | O(n) space
pub fn climb_stairs_dp(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

// O(n) time | O(1) space
pub fn climb_stairs(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut fst = 1;
    let mut snd = 1;

    for _ in 2..=n {
        let tmp = fst + snd;
        fst = snd;
        snd = tmp;
    }

    snd
}

#[cfg(test)]
mod tests {
    use crate::dynamic::climbing_stairs::climb_stairs_dp;
    use crate::dynamic::climbing_stairs::climb_stairs;

    #[test]
    fn it_climb_stairs_dp() {
        assert_eq!(climb_stairs_dp(0), 0);
        assert_eq!(climb_stairs_dp(1), 1);
        assert_eq!(climb_stairs_dp(2), 2);
        assert_eq!(climb_stairs_dp(3), 3);
        assert_eq!(climb_stairs_dp(4), 5);
        assert_eq!(climb_stairs_dp(5), 8);
    }

    #[test]
    fn it_climb_stairs() {
        assert_eq!(climb_stairs(0), 0);
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}