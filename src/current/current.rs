
pub fn min_swaps_required(str: &str) -> Option<i32> {
    None
}


#[cfg(test)]
mod tests {

    use super::min_swaps_required;

    #[test]
    fn it_min_swaps_required() {

        assert_eq!(min_swaps_required("0100101"), Some(2));
    }

    #[test]
    fn it_no_way_to_make_palindrom() {

        assert_eq!(min_swaps_required("1110"), None);
    }

    #[test]
    fn it_one_swap() {
        assert_eq!(min_swaps_required("11101"), Some(1));
    }
}