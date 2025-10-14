
pub fn median(arr1: &[i32], arr2: &[i32]) -> f64 {
    0.0
}

pub fn median1(arr1: &[i32], arr2: &[i32]) -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::{median, median1};

    const ARR1: &[i32] = &[1,2,3,4,5,6];
    const ARR2: &[i32] = &[7,8,9,10,11,12];
    const ARR3: &[i32] = &[1,2,3,4,5,6,13];

    const EXP_RES: f64 = 6.5;
    const EXP_RES1: f64 = 7.0;

    #[test]
    fn it_median() {
        assert_eq!(median(ARR1, ARR2), EXP_RES);
    }

    #[test]
    fn it_median_one() {
        assert_eq!(median(ARR3, ARR2), EXP_RES1);
    }

    #[test]
    fn it_median1() {
        assert_eq!(median1(ARR1, ARR2), EXP_RES);
    }

    #[test]
    fn it_median1_one() {
        assert_eq!(median1(ARR3, ARR2), EXP_RES1);
    }
}