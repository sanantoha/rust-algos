
// O(log(n)) time | O(log(n)) space
pub fn pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    } else if n == 1 {
        return x;
    } else if n < 0 {
        return pow(1f64 / x, -n);
    } else if n % 2 == 0 {
        let y = pow(x, n / 2);
        y * y
    } else {
        return x * pow(x, n - 1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(pow(4f64, 2), 16.0);
    }

    #[test]
    fn test_pow_case1() {
        assert_eq!(pow(2f64, 4), 16.0);
    }

    #[test]
    fn test_pow_case2() {
        assert_eq!(pow(2f64, -2), 0.25);
    }

    #[test]
    fn test_pow_case3() {
        let epsilon = 1e-9;
        let expected = 2.1_f64.powi(3);
        println!("{}", expected);

        let res = pow(2.1, 3);

        assert!((res - expected).abs() < epsilon,
                "Expected {} to be approximately equal to {}, but they differ by {}",
                res,
                expected,
                (res - expected).abs()
        );
    }
}