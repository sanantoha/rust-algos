
pub fn pow(x: f64, n: i32) -> f64 {
    0.0
}

pub fn sort_k_sorted_array(arr: &mut [i32], k: i32) -> Vec<i32> {
    vec![]
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


    #[test]
    fn basic() {
        let mut arr: Vec<i32> = vec![3, 2, 1, 5, 4, 7, 6, 5];

        assert_eq!(sort_k_sorted_array(&mut arr, 3), vec![1, 2, 3, 4, 5, 5, 6, 7]);
    }
}