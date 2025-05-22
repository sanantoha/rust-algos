
pub fn sqrt(n: i32) -> i32 {
    0
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