
pub fn max_profit(arr: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn it_max_profit() {

        assert_eq!(max_profit(&vec![7, 1, 5, 3, 6, 4]), 5);
    }
}