
// O(n * d) time | O(n) space
pub fn min_number_of_coins_for_change(n: usize, denoms: &[usize]) -> i32 {

    let mut mnc = vec![i32::MAX; n + 1];
    mnc[0] = 0;

    for &denom in denoms {
        for amount in 1..=n {
            if amount >= denom {
                let mut to_change = mnc[amount - denom];
                if to_change != i32::MAX {
                    to_change += 1;
                }

                mnc[amount] = to_change.min(mnc[amount]);
            }
        }
    }

    if mnc[n] == i32::MAX {
        -1
    } else {
        mnc[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::dynamic::min_number_of_coins_for_change::min_number_of_coins_for_change;

    #[test]
    fn test_min_number_of_coins_for_change() {
        assert_eq!(min_number_of_coins_for_change(7, &[1, 5, 10]), 3);
    }
}