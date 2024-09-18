
// O(n * d) time | O(n) space
pub fn numbers_of_ways_to_make_change(n: usize, denoms: &[usize]) -> i32 {
    if denoms.is_empty() || n <= 0 {
        return 0;
    }

    let mut nwmc = vec![0; n + 1];
    nwmc[0] = 1;

    for denom in denoms {
        for amount in 1..=n {
            if amount >= *denom {
                nwmc[amount] += nwmc[amount - *denom];
            }
        }
    }

    nwmc[n]
}

#[cfg(test)]
mod tests {
    use crate::dynamic::numbers_of_ways_to_make_change::numbers_of_ways_to_make_change;

    #[test]
    fn test_numbers_of_ways_to_make_change() {

        assert_eq!(numbers_of_ways_to_make_change(6, &[1, 5]), 2);
    }
}