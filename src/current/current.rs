
pub fn min_rewards(arr: &[i32]) -> i32 {
    0
}

pub fn min_rewards1(arr: &[i32]) -> i32 {
    0
}

pub fn min_rewards2(arr: &[i32]) -> i32 {
    0
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