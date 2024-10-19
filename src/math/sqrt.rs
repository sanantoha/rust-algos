
// O(log(n)) time | O(1) space
pub fn sqrt(n: i32) -> i32 {

    let mut l: i64 = 0;
    let mut r: i64 = n as i64;

    while l <= r {
        let m = l + (r - l) / 2;

        if m * m <= n as i64 {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    (l - 1) as i32
}

#[cfg(test)]
mod tests {

    use super::sqrt;

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(16), 4);
    }

    #[test]
    fn test_sqrt_case1() {
        assert_eq!(sqrt(8), 2);
    }
}