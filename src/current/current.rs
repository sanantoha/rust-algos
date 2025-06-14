

pub fn numbers_of_ways_to_make_change(n: usize, denoms: &[usize]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::numbers_of_ways_to_make_change;

    #[test]
    fn test_numbers_of_ways_to_make_change() {

        assert_eq!(numbers_of_ways_to_make_change(6, &[1, 5]), 2);
    }
}