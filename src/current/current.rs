
pub fn climb_stairs_dp(n: usize) -> usize {
    0
}

pub fn climb_stairs(n: usize) -> usize {
    0
}


#[cfg(test)]
mod tests {
    use super::climb_stairs_dp;
    use super::climb_stairs;

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