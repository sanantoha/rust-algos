#[cfg(test)]
mod tests {

    use super::super::super::super::arrays::find_pivot_index;

    #[test]
    fn test_find_pivot_index() {

        assert_eq!(find_pivot_index::find_pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(find_pivot_index::find_pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(find_pivot_index::find_pivot_index(vec![2, 1, -1]), 0);
    }
}