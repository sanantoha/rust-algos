
pub fn find_peak(arr: &[i32]) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::find_peak;

    #[test]
    fn test_find_peak() {
        let res = find_peak(&[1,2,1,3,4,5,6,5,4,3,2,1]);
        println!("{:?}", res);

        assert_eq!(res, Some(6));
    }
}