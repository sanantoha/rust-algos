
pub fn water_area(heights: &[i32]) -> i32 {
    0
}

pub fn water_area1(heights: &[i32]) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::{water_area, water_area1};

    const HEIGHTS: &[i32] = &[0, 8, 0, 0, 5, 0, 0, 10, 0, 0, 1, 1, 0, 3];

    const EXP: i32 = 48;

    #[test]
    fn test_water_area() {
        assert_eq!(water_area(HEIGHTS), EXP)
    }

    #[test]
    fn test_water_area1() {
        assert_eq!(water_area1(HEIGHTS), EXP)
    }
}