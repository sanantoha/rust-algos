
// O(n) time | O(n) space
pub fn water_area(heights: &[i32]) -> i32 {
    if heights.is_empty() {
        return 0;
    }

    let mut water = vec![0i32; heights.len()];

    let mut left_max = 0;

    for i in 0..heights.len() {
        water[i] = left_max;
        left_max = left_max.max(heights[i]);
    }

    let mut right_max = 0;

    for i in (0..heights.len()).rev() {
        let min_height = water[i].min(right_max);
        if min_height < heights[i] {
            water[i] = 0;
        } else {
            water[i] = min_height - heights[i];
        }
        right_max = right_max.max(heights[i]);
    }

    water.iter().sum()
}

// O(n) time | O(1) space
pub fn water_area1(heights: &[i32]) -> i32 {
    if heights.is_empty() {
        return 0;
    }

    let mut l = 0;
    let mut r = heights.len() - 1;
    let mut left_max = heights[l];
    let mut right_max = heights[r];

    let mut area = 0;

    while l < r {
        if left_max < right_max {
            l += 1;
            left_max = left_max.max(heights[l]);
            area += left_max - heights[l];
        } else {
            r -= 1;
            right_max = right_max.max(heights[r]);
            area += right_max - heights[r];
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use crate::dynamic::water_area::{water_area, water_area1};

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