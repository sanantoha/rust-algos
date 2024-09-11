
// O(n) time | O(1) space
pub fn max_profit(prices: &[i32]) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut max_profit = 0;
    let mut min_val = prices[0];

    for num in prices {
        max_profit = max_profit.max(*num - min_val);
        min_val = min_val.min(*num);
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use crate::dynamic::best_time_to_buy_and_sell_stocks::max_profit;

    #[test]
    fn it_max_profit() {

        assert_eq!(max_profit(&vec![7, 1, 5, 3, 6, 4]), 5);
    }
}