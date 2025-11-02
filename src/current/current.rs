
pub fn find_pivot_index(_arr: &[i32]) -> Option<usize> {
    None
}


#[cfg(test)]
mod tests {
    use super::find_pivot_index;

    #[test]
    fn test_find_pivot_index() {

        assert_eq!(find_pivot_index(&[1, 7, 3, 6, 5, 6]), Some(3));
        assert_eq!(find_pivot_index(&[1, 2, 3]), None);
        assert_eq!(find_pivot_index(&[2, 1, -1]), Some(0));
    }
}