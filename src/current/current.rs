
pub fn largest_range(arr: &[i32]) -> (i32, i32) {
    (0, 0)
}

pub fn largest_range1(arr: &[i32]) -> (i32, i32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::largest_range;
    use super::largest_range1;

    const ARR: &[i32] = &[1, 11, 3, 0, 15, 5, 2, 4, 10, 7, 12, 6];

    #[test]
    fn it_largest_range() {
        assert_eq!(largest_range(ARR), (0, 7));
    }

    #[test]
    fn it_largest_range1() {
        let res = largest_range1(ARR);
        println!("{res:?}");
        assert_eq!(res, (0, 7));
    }

}