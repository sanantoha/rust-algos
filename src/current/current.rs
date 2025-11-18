
pub fn min_number_of_coins_for_change(n: usize, denoms: &[usize]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::min_number_of_coins_for_change;

    #[test]
    fn test_min_number_of_coins_for_change() {
        assert_eq!(min_number_of_coins_for_change(7, &[1, 5, 10]), 3);
    }
}